

# CESILPy - A CESIL interpreter for Python
# Yes, it's fairly basic
# (c) 2016, Alan James Salmoni

import sys, os

class CESILObj(object):
    def __init__(self, fname):
        try:
            fin = open(fname,'r')
        except IOError:
            print("### ERROR: CANNOT OPEN "+fname+" ###")
            self.halt_good()
        try:
            script = fin.read()
        except:
            print("### ERROR: CANNOT READ FILE "+fname+" ###")
            self.halt_good()
        fin.close()
        self.fname = fname
        self.script = script.split(os.linesep)
        self.data = []
        self.dataindex = 0
        self.ParseScript()
        self.ParseData()
        self.commands = ["LOAD","STORE","JUMP","JINEG","JIZERO","PRINT","OUT",
                "IN","LINE","HALT","ADD","SUBTRACT","MULTIPLY","DIVIDE"]
        self.acc = 0 # accumulator
        self.labels = {}
        self.variables = {}
        self.ParseLabels()
        self.ParseVariables()
        self.RunScript()

    def ParseScript2(self):
        self.newscript = []
        for line in self.script:
            pass

    def ParseScript(self):
        self.newscript = []
        for line in self.script:
            line_len = len(line)
            if len(line) > 0:
                # is not an empty line
                if line[0] == '*':
                    pass
                else:
                    if line_len < 9:
                        # we have a label space only
                        word1 = line.strip(' ')
                        word2 = ''
                        word3 = ''
                    elif line_len < 17:
                        # we have label space and command only
                        word1 = line[0:8].strip(' ')
                        word2 = line[8:16].strip(' ')
                        word3 = ''
                    else:
                        word1 = line[0:8].strip(' ')
                        word2 = line[8:16].strip(' ')
                        word3 = line[16:]
                        if '"' not in word3:
                            # is not a string
                            word3 = word3.strip(' ')
                        else:
                            word3 = word3.lstrip()
                    self.newscript.append([word1, word2, word3])

    def ParseData(self):
        idx = 0
        for idx, line in enumerate(self.script):
            try:
                if line[8] == "%":
                    datablock = self.script[idx+1:]
                    for dataline in datablock:
                        data = dataline.strip()
                        if len(data) > 0:
                            self.data.append(data)
            except IndexError:
                pass

    def ParseLabels(self):
        """
        This runs through and extracts all label names and stores them with the
        line number.
        Duplicates are not allowed and cause termination.
        """
        for idx, line in enumerate(self.newscript):
            if len(line[0]) > 0: # this filter might not be needed
                initword = line[0]
                if initword not in self.labels.keys():
                    self.labels[initword] = idx
                else:
                    print("### ERROR ### DUPLICATE LABEL ON LINE "+str(idx)+line[0])
                    self.halt_good()

    def ParseVariables(self):
        """
        This runs through and extracts all variable names and initialised them
        All variables are initialised to zero (0)
        """
        for idx, line in enumerate(self.newscript):
            try:
                poss_var = line[2]
                if '"' not in poss_var:
                    if poss_var not in self.labels:
                        try:
                            val = int(poss_var)
                        except ValueError: # should not be integer
                            self.variables[poss_var] = 0
            except IndexError:
                pass

    def halt_good(self):
        print()
        sys.exit()

    def RunScript(self):
        self.linenum = 0
        while True:
            line = self.newscript[self.linenum]
            #print ("LN = ",str(self.linenum+1))
            old_line = self.linenum
            res = self.parse_line(line)
            if old_line == self.linenum:
                self.linenum = self.linenum + 1
                if self.linenum > len(self.script):
                    self.halt_good()

    def GetNumber(self, operand):
        try:
            value = int(operand)
        except ValueError:
            value = self.variables[operand]
        return value

    def parse_line(self, line):
        # Run through commands in turn
        label = line[0]
        operator = line[1]
        operand = line[2]
        if operator == "HALT":
            self.halt_good()
        elif operator == "IN":
            try:
                data = self.data[self.dataindex]
            except IndexError:
                print ("### ERROR: Not enough data to be read ###")
                self.halt_good()
            self.dataindex += 1
            try:
                self.acc = int(data)
            except ValueError:
                print ("### ERROR: Non-integer data [ %s ]###"%data)
                self.halt_good()
        elif operator == "OUT":
            print (str(self.acc), end="")
        elif operator == "LINE":
            print ("\n",end="")
        elif operator == "LOAD":
            self.acc = self.GetNumber(operand)
        elif operator == "STORE":
            self.variables[operand] = self.acc
        elif operator == "PRINT":
            print (operand.strip('"').strip('\n'), end="")
        elif (operator == "ADD") or (operator == "SUBTRACT") or (operator == "MULTIPLY") or (operator == "DIVIDE"):
            value = self.GetNumber(operand)
            if operator == "ADD":
                self.acc = self.acc + value
            elif operator == "SUBTRACT":
                self.acc = self.acc - value
            elif operator == "MULTIPLY":
                self.acc = self.acc * value
            elif operator == "DIVIDE":
                if value == 0:
                    print ("### ERROR: DIVIDE BY ZERO LINE "+str(self.linenum)+" ###")
                    self.halt_good()
                else:
                    self.acc = int(self.acc / value)
        elif operator == "JUMP":
            self.linenum = self.labels[operand]
        elif operator == "JIZERO":
            if self.acc == 0:
                self.linenum = self.labels[operand]
        elif operator == "JINEG":
            if self.acc < 0:
                self.linenum = self.labels[operand]

if __name__ == "__main__":
    try:
        fname = sys.argv[1]
    except:
        print("### ERROR: NO FILENAME SPECIFIED ###")
        sys.exit()
    CESIL = CESILObj(fname)