import Arm2D
import PyInfo

class Compiler(SysObj):
    obj = PyInfo.PyObj()
    PyInfo.PyMethod()
    PyInfo.PyClass()
    line = Arm2D.Line()

    def build(pythonApiPath: str, outputPath: str) -> int:
        pass

    def analyzestr(pythonApiPath: str) -> int:
        pass

    def analyzeLine(line: str) -> int:
        pass
