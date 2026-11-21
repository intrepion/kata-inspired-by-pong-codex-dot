import tempfile
import os
import stat
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch

from native_release import REGISTRY, check_checkout, isolated_lock, privacy, remove_scratch, run
from native_source import CONFIG, prepare

NAME, VERSION, LOCK, DIGEST, *_ = CONFIG["sdl3"]
ENTRY = f'name = "{NAME}"\nversion = "{VERSION}"\nsource = "{REGISTRY}"\nchecksum = "{DIGEST}"\n'
LOCKED = f'version = 4\n\n[[package]]\n{ENTRY}dependencies = ["x"]\n\n[[package]]\nname = "x"\nversion = "1.0.0"\n'.encode()


class NativeReleaseTests(unittest.TestCase):
    def test_only_registry_identity_changes(self):
        actual = isolated_lock(LOCKED, NAME, VERSION, DIGEST)
        expected = LOCKED.replace(f'source = "{REGISTRY}"\nchecksum = "{DIGEST}"\n'.encode(), b"")
        self.assertEqual(actual, expected)

    def test_tampered_and_duplicate_lock_rejected(self):
        for data in (LOCKED.replace(DIGEST.encode(), b"0" * 64), LOCKED + b"\n[[package]]\n" + ENTRY.encode()):
            with self.assertRaises(ValueError):
                isolated_lock(data, NAME, VERSION, DIGEST)

    def test_dirty_checkout_rejected_even_when_head_and_tree_match(self):
        repo = Path.cwd()
        outputs = [str(repo).encode(), b"a" * 40, b"b" * 40, b"?? foreign\n"]
        with patch("native_release.git", side_effect=outputs), self.assertRaises(ValueError):
            check_checkout(repo, "a" * 40, "b" * 40)

    def test_preexisting_output_kept(self):
        with tempfile.TemporaryDirectory() as temp:
            d = Path(temp)
            repo = d / "repo"; repo.mkdir()
            archive = d / "input"; archive.touch()
            out = d / "existing"; out.mkdir()
            proof = out / "existing.txt"; proof.write_text("untouched")
            args = SimpleNamespace(repo=repo, crate=archive, output=out)
            with self.assertRaises(ValueError):
                run(args)
            self.assertEqual(proof.read_text(), "untouched")

    def test_tampered_archive_rejected_before_extraction(self):
        with tempfile.TemporaryDirectory() as temp:
            r = Path(temp)
            lock = r / LOCK; lock.parent.mkdir(parents=True)
            lock.write_bytes(LOCKED)
            archive = r / "tampered"; archive.write_bytes(b"untrusted byte")
            output = r / "new"
            with self.assertRaisesRegex(ValueError, "archive digest"):
                prepare("sdl3", r, archive, output)
            self.assertFalse(output.exists())

    def test_privacy_catches_utf16le_and_ignores_approved_logical_path(self):
        with self.assertRaises(ValueError):
            privacy("c:\\users\\alice\\native.c".encode("utf-16le"), [])
        privacy(b"/pong-sdl3-source/native.c", [])

    @unittest.skipUnless(os.name == "nt", "Windows read-only attribute")
    def test_own_readonly_checkout_pack_can_be_removed(self):
        with tempfile.TemporaryDirectory() as temp:
            d = Path(temp).resolve()
            own = d / "scratch"; own.mkdir()
            pack = own / "git-created-pack"; pack.write_bytes(b"test")
            pack.chmod(stat.S_IREAD)
            sentinel = d / "outside"; sentinel.touch()
            remove_scratch(own)
            self.assertFalse(own.exists())
            self.assertTrue(sentinel.is_file())
