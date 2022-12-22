// Advent of Code 2015, Day 3
// Perfectly Spherical Houses in a Vacuum

#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

int main()
{
  std::string temp, input = "";
  std::ifstream inFile;
  inFile.open("Day 3 - Input.txt");

  while (inFile >> temp)
    input += temp;

  unsigned int ups = 0, downs = 0, lefts = 0, rights = 0;
  for (const auto& x : input)
  {
    if (x == '^')
      ++ups;
    else if (x == 'v')
      ++downs;
    else if (x == '<')
      ++lefts;
    else if (x == '>')
      ++rights;
  }

  unsigned int maxDim = std::max(std::max(ups, downs), std::max(lefts, rights));

  std::vector<std::vector<bool>> houseGrid;
  for (unsigned int i = 0; i != (maxDim*2 + 1); ++i)
  {
    std::vector<bool> tempRow;

    for (unsigned int j = 0; j != (maxDim*2 +1); ++j)
      tempRow.push_back(false);

    houseGrid.push_back(tempRow);
  }

  // Part 1

  // decltype(houseGrid.size()) xpos = houseGrid.size()/2, ypos = xpos;
  //
  // for (const auto& x : input)
  //   {
  //     //std::cout << "xpos: " << xpos << "; ypos: " << ypos << "\n";
  //     houseGrid[xpos][ypos] = true;
  //
  //     if (x == '^')
  //       ++xpos;
  //     else if (x == 'v')
  //       --xpos;
  //     else if (x == '<')
  //       --ypos;
  //     else if (x == '>')
  //       ++ypos;
  //   }
  //
  // decltype(houseGrid.size()) houseSum = 0;
  //
  // for (const auto& i : houseGrid)
  //   for (const auto& j : i)
  //     houseSum += j;
  //
  //
  // std::cout << houseSum << " houses received at least one gift.\n";

  // Part 2

  decltype(houseGrid.size()) xposA = houseGrid.size()/2, yposA = xposA, xposB = xposA, yposB = yposA;

  bool even = true;

  for (const auto& x : input)
    {
      //std::cout << "xpos: " << xpos << "; ypos: " << ypos << "\n";
      houseGrid[xposA][yposA] = true;
      houseGrid[xposB][yposB] = true;

      if (even)
      {
        even = !even;

        if (x == '^')
          ++xposA;
        else if (x == 'v')
          --xposA;
        else if (x == '<')
          --yposA;
        else if (x == '>')
          ++yposA;
      }

      else
      {
        even = !even;

        if (x == '^')
          ++xposB;
        else if (x == 'v')
          --xposB;
        else if (x == '<')
          --yposB;
        else if (x == '>')
          ++yposB;
      }
    }

  decltype(houseGrid.size()) houseSum = 0;

  for (const auto& i : houseGrid)
    for (const auto& j : i)
      houseSum += j;


  std::cout << houseSum << " houses received at least one gift.\n";

  return 0;
}
