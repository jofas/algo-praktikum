import ctypes
from ctypes import c_void_p,c_int32,c_size_t,Structure

class CVertex(Structure):
    _fields_ = [("id",c_int32),
               ("x",c_int32),
               ("y",c_int32)]

    def __str__(self):
        return "{} {} {}".format(self.id,self.x,self.y)

class CArray(Structure):
    _fields_ = [("len",c_size_t),
                ("data",c_void_p)]

    def __str__(self):
        x = ""
        for i in range (0,int(self.len)) :
            x += "Hallo "
        return x

class Coordinates(Structure):
    _fields_ = [("x", c_int32),
                ("y", c_int32)]
    def __str__(self):
        return "({},{})".format(self.x,self.y)

lib = ctypes.cdll.LoadLibrary("target/debug/libgraphs.so")
lib.get_graphs.restype = CVertex



def getGraphs():
    ptr = lib.get_graphs()
    return ctypes.cast(ptr, ctypes.c_char_p).value.decode("utf-8")

#gs = getGraphs().split(",")[:-1]
x = lib.get_graphs()
print(x)
