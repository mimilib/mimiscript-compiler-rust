class PyObj(BaseObj):

    def importClass(className: str, fp: FILE):
        pass

    def newObj(objName: str, className: str, fp: FILE):
        pass

    def makeInlcude(fp: FILE):
        pass

    def makeNewObj(fp: FILE):
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
    def makeMethodFun(fp: FILE):
        pass

    def makeMethodDeclear(fp: FILE):
        pass

    def makeMethodDefine(fp: FILE):
        pass


class Compiler(SysObj):
    obj = PyObj()
    PyMethod()
    PyClass()

    def build(pythonApiPath: str, outputPath: str) -> int:
        pass

    def analyzeFile(pythonApiPath: str) -> int:
        pass

    def analyzeLine(line: str) -> int:
        pass
