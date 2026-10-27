#!/usr/bin/env python3
"""Copy the used wasm32 wire layouts and import declarations into Rust.
Input: the pinned wago-vulkan abi/wire.h and the Vulkan SDK's vulkan_core.h.
This is a source-generation tool, not part of the normal build.
"""
import pathlib
import re
import sys

wire = pathlib.Path(sys.argv[1]).read_text()
constants = pathlib.Path(sys.argv[2]).read_text()
# The C prototype is retained outside the source tree only during the initial port.
prototype = pathlib.Path(sys.argv[3]).read_text()
imports = pathlib.Path(sys.argv[4]).read_text()
blocks = dict((m.group(2), m.group(0)) for m in re.finditer(
    r'typedef struct \{((?:(?!typedef).)*?)\} wv32_(\w+);', wire, re.S))
needed = set(re.findall(r'wv32_(\w+)', prototype))
while True:
    expanded = needed | {t for n in needed for t in re.findall(r'wv32_(\w+)', blocks[n]) if t != n}
    if expanded == needed:
        break
    needed = expanded
out = ['// Generated subset of wago-vulkan a3943c4 abi/wire.h. See vendor/ for license.',
       '#![allow(non_snake_case)]']
maptype = {'uint32_t': 'u32', 'uint64_t': 'u64', 'uint8_t': 'u8', 'int32_t': 'i32', 'float': 'f32', 'char': 'u8', 'size_t': 'u32'}
for name in sorted(needed):
    body = re.search(r'typedef struct \{(.*?)\}', blocks[name], re.S).group(1)
    out.extend(['#[repr(C)]', '#[derive(Clone, Copy)]', f'pub struct {name} {{'])
    for line in body.strip().splitlines():
        m = re.fullmatch(r'\s*(\w+) (\w+)(?:\[(\d+)\])?;', line)
        typ, field, size = m.groups()
        typ = maptype.get(typ, typ.removeprefix('wv32_'))
        if size:
            typ = f'[{typ}; {size}]'
        out.append(f'    pub {field}: {typ},')
    out.append('}')
    for m in re.finditer(r'_Static_assert\((.*?) == (\d+), "' + name + r'[^"\n]*"\);', wire):
        expr, size = m.groups()
        expr = expr.replace('sizeof(wv32_' + name + ')', 'core::mem::size_of::<' + name + '>()')
        expr = expr.replace('offsetof(wv32_' + name + ', ', 'core::mem::offset_of!(' + name + ', ')
        out.append(f'const _: () = assert!({expr} == {size});')
for name in sorted(set(re.findall(r'\bVK_\w+', prototype))):
    if name == 'VK_API_VERSION_1_0':
        value = '4194304'
    elif name in ('VK_SUBPASS_EXTERNAL', 'VK_QUEUE_FAMILY_IGNORED'):
        value = '0xffffffff'
    else:
        m = re.search(r'\b' + name + r'\s*=\s*(0x[0-9a-fA-F]+|-?\d+)\b', constants)
        if not m:
            raise ValueError(name)
        value = m.group(1)
    typ = 'i32' if name == 'VK_SUCCESS' else 'u32'
    out.append(f'pub const {name}: {typ} = {value};')
pathlib.Path('guest/src/wire.rs').write_text('\n'.join(out) + '\n')
out = ['// Named imports from wago-vulkan ABI version 1.', '#![allow(non_snake_case)]',
       'use core::ffi::c_void;', '#[link(wasm_import_module = "vulkan.wasm32")]', 'extern "C" {']
maptype = {'int32_t':'i32', 'uint32_t':'u32', 'uint64_t':'u64', 'void':'()', 'void*':'*mut c_void',
           'const void*':'*const c_void', 'uint64_t*':'*mut u64', 'uint32_t*':'*mut u32'}
for m in re.finditer(r'^IMPORT\((\w+), (\w+), (.*?)\);', imports, re.M):
    ret, name, params = m.groups()
    types = [] if params == 'void' else [maptype[t.strip()] for t in params.split(',')]
    args = ', '.join(f'a{i}: {t}' for i, t in enumerate(types))
    out.append(f'    pub fn {name}({args}) -> {maptype[ret]};')
out.append('}')
pathlib.Path('guest/src/imports.rs').write_text('\n'.join(out) + '\n')
