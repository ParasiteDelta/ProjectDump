import std/random
import std/strformat
import std/strutils

proc sfmt(inp: string): string =
   result = "\n" & inp & "\n"

proc getInpNum(prompt: string): int =
   echo sfmt(prompt)
   result = parseInt(readLine(stdin))

proc exportWorksheet(op: string, main: string, ins: string) =
   var output = ""

   case op:
      of "+":
         output = "Addition"
      of "-":
         output = "Subtraction"
      of "*":
         output = "Multiplication"
      else:
         echo "ERR: Unrecognized Operator!"

   writeFile(fmt"NRQuiz_{output}.txt", main)
   writeFile(fmt"NRQuiz_{output}_Ins.txt", ins)

proc makeWorksheet(op: string) =
   var
      num_prob = getInpNum("Please enter the number of problems you would like to generate:")
      num_low = getInpNum("Please enter the lowest possible number you want in the generation range:")
      num_high = getInpNum("Please enter the highest possible number in the generation range:")
      lnum = 0
      n1Last = 0
      n2Last = 0
      nsLast = 0
      usedNums: seq[int] = @[]
      sepPerc = int((num_high - num_low) / 10)
      final = ""
      finalIns = ""

   while lnum < num_prob:
      var
         num1 = rand(num_low..num_high)
         num2 = rand(num_low..num_high)
         numSum = 0
         i = 0

      while i < 1:
         case op:
            of "+":
               numSum = num1 + num2
            of "-":
               numSum = num1 - num2
            of "*":
               numSum = num1 * num2
            else:
               echo "ERR: Unrecognized operator!"
               break

         randomize()

         if
            num1 in (num2 - sepPerc..num2 + sepPerc) or
            num1 in (n1Last - sepPerc..n1Last + sepPerc) or
            num1 in usedNums:
               num1 = rand(num_low..num_high)
               continue
         if
            num2 in (num1 - sepPerc..num1 + sepPerc) or
            num2 in (n2Last - sepPerc..n2Last + sepPerc) or
            num2 in usedNums:
               num2 = rand(num_low..num_high)
               continue
         if
            numSum in (nsLast - sepPerc..nsLast + sepPerc) or
            numSum in usedNums:
               num1 = rand(num_low..num_high)
               num2 = rand(num_low..num_high)
               continue

         break

      var
         finalProb = $(lnum + 1) & ": " & $num1 & fmt" {op} " & $num2 & "\n"
         finalProbIns = $(lnum + 1) & ": " & $num1 & fmt" {op} " & $num2 & " = " & $numSum & "\n"

      final.add(finalProb)
      finalIns.add(finalProbIns)
      usedNums.add(num1)
      usedNums.add(num2)
      usedNums.add(numSum)
      n1Last = num1
      n2Last = num2
      nsLast = numSum
      inc lnum

   exportWorksheet(op, final, finalIns)

proc menu() =
   var usr_entry = ""

   while usr_entry != "4":
      echo sfmt("Welcome to NRQuiz!\nPlease select an option from below:\n 1: Generate Addition Worksheet\n 2: Generate Subtraction Worksheet\n 3: Generate Multiplication Worksheet\n 4: Exit")
      usr_entry = readLine(stdin)

      case parseInt(usr_entry):
         of 1:
            makeWorksheet("+")
         of 2:
            makeWorksheet("-")
         of 3:
            makeWorksheet("*")
         of 4:
            echo sfmt("Thanks for using NRQuiz!\nGoodbye!")
            quit(0)
         else:
            echo sfmt("ERR: Unknown option! Please try again!")

menu()