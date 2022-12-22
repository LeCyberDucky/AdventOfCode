// Advent of Code 2015, Day 1
// Not Quite Lisp

#include <fstream>
#include <iostream>

int main()
{
  std::ifstream inFile;
  inFile.open("Day 1 - Input.txt");

  char temp;
  int level = 0;
  unsigned int instructionCount = 0;
  bool basementReached = false;

  while (inFile >> temp)
    {
      if (!basementReached)
        ++instructionCount;

      if (temp == '(')
        ++level;
      else if (temp == ')')
        --level;

      if (level < 0)
        basementReached = true;
    }

  std::cout << "Santa should go to floor " << level << ".\n";
  std::cout << "The basement was reached at instrution " << instructionCount << ".\n";

  return 0;
}
