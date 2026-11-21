"""Regenerate or verify the Pong-specific CP437 PNG cells (stdlib only)."""
import argparse
from pathlib import Path
import re
import struct
import zlib

# Original 5x7 drawings for the HUD, encoded one 5-bit row per byte.
GLYPHS = {
    "A":"0e11111f111111", "B":"1e11111e11111e", "C":"0f10101010100f",
    "D":"1e11111111111e", "E":"1f10101e10101f", "F":"1f10101e101010",
    "G":"0f10101711110f", "H":"1111111f111111", "I":"0e04040404040e",
    "J":"0702020212120c", "K":"11121418141211", "L":"1010101010101f",
    "M":"111b1515111111", "N":"11191513111111", "O":"0e11111111110e",
    "P":"1e11111e101010", "Q":"0e11111115120d", "R":"1e11111e141211",
    "S":"0f10100e01011e", "T":"1f040404040404", "U":"1111111111110e",
    "V":"11111111110a04", "W":"1111111515150a", "X":"11110a040a1111",
    "Y":"11110a04040404", "Z":"1f01020408101f",
    "a":"00000e010f110f", "b":"10101e1111111e", "c":"00000f1010100f",
    "d":"01010f1111110f", "e":"00000e111f100f", "f":"0609081c080808",
    "g":"00000f11110f011e", "h":"10101e11111111", "i":"04000c0404040e",
    "j":"020006020202120c", "k":"10101214181412", "l":"0c04040404040e",
    "m":"00001a15151515", "n":"00001e11111111", "o":"00000e1111110e",
    "p":"00001e11111e1010", "q":"00000f11110f0101", "r":"00001619101010",
    "s":"00000f100e011e", "t":"08081e08080906", "u":"0000111111110f",
    "v":"00001111110a04", "w":"0000111115150a", "x":"0000110a040a11",
    "y":"00001111110f011e", "z":"00001f0204081f",
    "0":"0e11131519110e", "1":"040c040404040e", "2":"0e11010204081f",
    "3":"1e01010601111e", "4":"02060a121f0202", "5":"1f101e0101110e",
    "6":"0e101e1111110e", "7":"1f010204080808", "8":"0e11110e11110e",
    "9":"0e11110f01010e", " ":"", "|":"04040404040404", ".":"00000000000c0c",
    ":":"000c0c000c0c00", "-":"0000001f000000", "/":"01010204081010",
}
ASSET_DIR = Path(__file__).resolve().parent


def stored_zlib(data):
    # Deterministic uncompressed DEFLATE; zlib is used only for Adler-32.
    result = bytearray(b"\x78\x01")
    for start in range(0, len(data), 65535):
        block = data[start : start + 65535]
        count = len(block)
        result.append(int(start + count == len(data)))
        result += struct.pack("<HH", count, count ^ 0xFFFF) + block
    return result + struct.pack(">I", zlib.adler32(data))


def chunk(tag, data):
    return struct.pack(">I", len(data)) + tag + data + struct.pack(">I", zlib.crc32(tag + data))


def atlas(height, glyphs, yscale, offset):
    w, h = 128, height * 16
    # PNG filter byte zero per row, transparent black away from glyphs.
    stride = 1 + w * 4
    pixels = bytearray(stride * h)
    for ch, encoded in glyphs.items():
        code = ord(ch)  # ASCII positions equal the CP437 slots we use.
        x0, y0 = (code % 16) * 8 + 1, (code // 16) * height + offset
        for y, bits in enumerate(bytes.fromhex(encoded)):
            assert bits < 32 and (y * yscale + offset + yscale) <= height
            for x in range(5):
                if bits & (16 >> x):
                    for dy in range(yscale):
                        p = (y0 + y * yscale + dy) * stride + 1 + (x0 + x) * 4
                        pixels[p : p + 4] = b"\xff" * 4
    return b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 6, 0, 0, 0)) + chunk(b"IDAT", stored_zlib(pixels)) + chunk(b"IEND", b"")


def verify_usage():
    source = (ASSET_DIR.parent / "src" / "main.rs").read_text()
    hud = source.split("fn header_lines", 1)[1].split("impl GameState", 1)[0]
    strings = re.findall(r'"([^"]*)"', hud)
    assert strings
    missing = set("".join(strings) + "0123456789") - set("{}") - GLYPHS.keys()
    assert not missing, f"missing HUD glyphs: {sorted(missing)}"
    assert set(re.findall(r"to_cp437\('(.)'\)", source)) == {" ", "|"}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="verify assets without writing")
    args = parser.parse_args()
    verify_usage()
    assets = {
        "pong-court8x8.png": atlas(8, {"|": "0404040404040404"}, 1, 0),
        "pong-header8x16.png": atlas(16, GLYPHS, 2, 0),
    }
    for name, image in assets.items():
        path = ASSET_DIR / name
        if args.check:
            if path.read_bytes() != image:
                raise SystemExit(f"{name} does not match the authored source")
        else:
            path.write_bytes(image)


if __name__ == "__main__":
    main()
