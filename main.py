import Arm2D

class PyObj(BaseObj):

    def importClass(className: str, fp: str):
        pass

    def newObj(objName: str, className: str, fp: str):
        pass

    def makeInlcude(fp: str):
        pass

    def makeNewObj(fp: str):
        pass

    def getInclude() -> str:
        pass


class PyClass(SysObj):
    def setSuper(superClassName: str):
        pass

    def makeApi(path: str):
        pass

    def makeHead(path: str):
        pass


class PyMethod (SysObj):
    def makeMethodFun(fp: str):
        pass

    def makeMethodDeclear(fp: str):
        pass

    def makeMethodDefine(fp: str):
        pass


class Compiler(SysObj):
    obj = PyObj()
    PyMethod()
    PyClass()
    line = Arm2D.Line()

    def build(pythonApiPath: str, outputPath: str) -> int:
        pass

    def analyzestr(pythonApiPath: str) -> int:
        pass

    def analyzeLine(line: str) -> int:
        pass
