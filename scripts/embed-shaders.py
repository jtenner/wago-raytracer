#!/usr/bin/env python3
import pathlib
import struct

out = ['// Generated from the GLSL sources by make. Do not edit.']
for name, path in [('vertex', 'guest/build/fullscreen.spv'), ('fragment', 'guest/build/present.spv')]:
    data = pathlib.Path(path).read_bytes()
    words = struct.unpack('<' + 'I' * (len(data) // 4), data)
    out.append('static const uint32_t ' + name + '_spv[] = {')
    for i in range(0, len(words), 8):
        out.append('    ' + ','.join('0x%08x' % w for w in words[i:i+8]) + ',')
    out.append('};')
pathlib.Path('guest/build/shaders.h').write_text('\n'.join(out) + '\n')
