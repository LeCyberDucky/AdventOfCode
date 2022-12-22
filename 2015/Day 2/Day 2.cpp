// Advent of Code 2015, Day 2
// I Was Told There Would Be No Math

#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <sstream>

int main()
{
  std::ifstream inFile;
  inFile.open("Day 2 - Input.txt");

  unsigned int a, b, c;
  uint64_t totalPaper = 0, totalRibbon = 0;
  std::string line;
  char temp;

  while (inFile >> a >> temp >> b >> temp >> c)
  {
    // Paper
    unsigned int alpha = a*b, beta = a*c, gamma = b*c;
    totalPaper += (2*alpha + 2*beta + 2*gamma);
    totalPaper += std::min(std::min(alpha, beta), std::min(alpha, gamma));

    // Ribbon
    unsigned int min, mid;
    min = std::min(std::min(a, b), std::min(a, c));
    mid = std::max(std::min(a, b), std::min(std::max(a, b), c));

    totalRibbon += 2 * (min + mid);
    totalRibbon += a * b * c;
  }

  std::cout << "Total paper needed: " << totalPaper << "sqft.\n";
  std::cout << "Total ribbon needed: " << totalRibbon << "ft.\n";

  // while (inFile >> line)
  // {
  //   std::stringstream wordStream(line);
  //
  //   wordStream >> a >> temp >> b >> temp >> c;
  //   std::cout << a << "\n";
  // }

  return 0;
}
