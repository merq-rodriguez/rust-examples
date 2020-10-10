from ctypes import cdll

lib = cdll.LoadLibrary('target/release/libembeber.so')

def test_processar():
    lib.procesar()

def test_rust_add():
    suma = lib.add(27, 15)
    print(suma)
    assert suma == 42

test_rust_add()
test_processar()