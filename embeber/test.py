from ctypes import cdll

lib = cdll.LoadLibrary('target/release/libembeber.so')

def test_rust_processar():
    lib.procesar()
    print("Terminado")

def test_rust_suma():
    suma = lib.suma(20, 15)
    print("Suma: "+str(suma))
    assert suma == 35

test_rust_suma()
test_rust_processar()