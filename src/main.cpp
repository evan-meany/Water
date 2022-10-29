#include "include/Lexer.h"
#include <iostream>

int main()
{
   Lexer lexer("test.wtr");
   lexer.printTokenList();
}