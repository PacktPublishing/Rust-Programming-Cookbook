from ctypes import cdll, c_char_p
from sys import platform

def build_lib_name(name):
    prefix = "lib"
    ext = "so"

    if platform == 'darwin':
        ext = 'dylib'
    elif platform == 'win32':
        prefix = ""
        ext = 'dll'

    return "{prefix}{name}.{ext}".format(prefix=prefix, name=name, ext=ext)

def main():
    lib = cdll.LoadLibrary(build_lib_name("digest"))
    lib.digest.restype = c_char_p
    print("SHA256 of Hello World =", lib.digest(b"Hello World"))

if __name__ == "__main__":
    main()