"""Synthetic readelf fixtures plus one read-only inspection of the Python ELF."""
import os
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch
from pathlib import Path
from elf_compat import ReportError, inspect, parse_readelf

HEADER='''ELF Header:\n  Class: ELF64\n  Data: 2's complement, little endian\n  Type: DYN (Shared object file)\n  Machine: AArch64\n'''
FIXTURE=HEADER+'''      [Requesting program interpreter: /lib/ld-linux-aarch64.so.1]\n 0x1 (NEEDED) Shared library: [libc.so.6]\n 0x1 (NEEDED) Shared library: [libstdc++.so.6]\n 0x2 (VERNEED) 0x1000\n 0x3 (VERNEEDNUM) 2\nVersion symbols section '.gnu.version' contains 3 entries:\n  000: 1 (*global*) 2 (GLIBC_2.88)\nVersion definition section '.gnu.version_d' contains 1 entry:\n  Name: GLIBC_9.99\nVersion needs section '.gnu.version_r' contains 2 entries:\n Addr: 0x0000 Offset: 0x0000 Link: 5 (.dynstr)\n  000000: Version: 1 File: libc.so.6 Cnt: 2\n  0x0010: Name: GLIBC_2.9 Flags: none Version: 2\n  0x0020: Name: GLIBC_2.10 Flags: none Version: 3\n  0x0030: Version: 1 File: libstdc++.so.6 Cnt: 3\n  0x0040: Name: GLIBCXX_3.4.29 Flags: none Version: 4\n  0x0050: Name: CXXABI_1.3.13 Flags: none Version: 5\n  0x0060: Name: GLIBC_ABI_DT_RELR Flags: none Version: 6\n'''
class ParseTests(unittest.TestCase):
    def test_required_versions_not_provided_and_numeric_sort(self):
        r=parse_readelf(FIXTURE);self.assertTrue(r['version_needs_section_present']);self.assertEqual(r['machine'],'AArch64');self.assertEqual(r['interpreter'],'/lib/ld-linux-aarch64.so.1')
        self.assertEqual(r['needed'],['libc.so.6','libstdc++.so.6']);self.assertEqual(r['version_requirements']['GLIBC'],['GLIBC_2.9','GLIBC_2.10'])
        self.assertEqual(r['minimum_symbol_versions']['GLIBC'],'GLIBC_2.10');self.assertEqual(r['minimum_symbol_versions']['GLIBCXX'],'GLIBCXX_3.4.29')
        self.assertEqual(r['minimum_symbol_versions']['CXXABI'],'CXXABI_1.3.13');self.assertEqual(r['special_version_requirements'],['GLIBC_ABI_DT_RELR'])
        self.assertNotIn('GLIBC_9.99',str(r));self.assertNotIn('GLIBC_2.88',str(r))
    def test_static_and_32_bit(self):
        r=parse_readelf(HEADER.replace('ELF64','ELF32')+'There is no dynamic section in this file.\nNo version information found in this file.\n')
        self.assertFalse(r['version_needs_section_present']);self.assertEqual(r['class'],'ELF32');self.assertIsNone(r['interpreter']);self.assertEqual(r['needed'],[]);self.assertIsNone(r['minimum_symbol_versions']['GLIBC'])
    def test_invalid_or_ambiguous_header(self):
        for text in ('not ELF',HEADER+'  Machine: Unknown\n'):
            with self.subTest(text=text),self.assertRaises(ReportError):parse_readelf(text)
    def test_actual_elf_is_only_read_and_revision_not_verified(self):
        path=Path(os.path.realpath(sys.executable)); before=path.stat().st_mtime_ns
        r=inspect(path,'a'*40);self.assertEqual(r['source_revision_claim'],'a'*40);self.assertFalse(r['source_revision_verified']);self.assertEqual(path.stat().st_mtime_ns,before)
        self.assertEqual(len(r['sha256']),64);self.assertIn('input not executed',r['method'])
    def test_truncated_or_unrecognized_needs_fails_closed(self):
        cases=[FIXTURE.rsplit('  0x0060:',1)[0],
               FIXTURE.replace('contains 2 entries:', 'contains 3 entries:'),
               FIXTURE.replace('Cnt: 3', 'Cnt: 4'),
               FIXTURE.replace('0x0040: Name:', '0x0040: Unexpected:'),
               FIXTURE.split("Version needs section")[0],
               FIXTURE.replace('(VERNEEDNUM) 2', '(VERNEEDNUM) 3'),
               FIXTURE.replace("Version needs section '.gnu.version_r' contains 2 entries:", "Version needs section malformed"),
               HEADER]
        for text in cases:
            with self.subTest(text=text[-90:]),self.assertRaises(ReportError):parse_readelf(text)
    def test_localized_caller_environment_is_overridden(self):
        binary=Path(os.path.realpath(sys.executable));real_run=subprocess.run
        with patch.dict(os.environ, {'LC_ALL':'fr_FR.UTF-8','LANG':'fr_FR.UTF-8'}), patch('elf_compat.subprocess.run', wraps=real_run) as call:
            inspect(binary)
            self.assertEqual(call.call_args.kwargs['env']['LC_ALL'],'C')
            self.assertEqual(call.call_args.kwargs['env']['LANG'],'C')
    def test_readelf_diagnostics_rejected(self):
        binary=Path(os.path.realpath(sys.executable))
        result=subprocess.CompletedProcess(args=[],returncode=0,stdout=HEADER+'No version information found in this file.\n',stderr='readelf: Warning: damaged data\n')
        with patch('elf_compat.subprocess.run',return_value=result),self.assertRaisesRegex(ReportError,'diagnostic'):
            inspect(binary)
    def test_fifo_rejected_without_waiting_for_writer(self):
        with tempfile.TemporaryDirectory() as t:
            fifo=Path(t)/'fifo';os.mkfifo(fifo)
            with self.assertRaisesRegex(ReportError,'not a regular file'):inspect(fifo)
    def test_reject_nonelf_symlink_and_invalid_revision(self):
        with tempfile.TemporaryDirectory() as t:
            p=Path(t)/'text';p.write_text('not executable');link=Path(t)/'link';link.symlink_to(p)
            for f,rev in [(p,None),(link,None),(p,'main')]:
                with self.subTest(f=f,rev=rev),self.assertRaises(ReportError):inspect(f,rev)

if __name__=='__main__':unittest.main()
