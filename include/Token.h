#include <string>

struct Token
{
   enum Type
   {
		WHITESPACE,
      NEWLINE,
      NUMBER,
      LETTER,
		EXPR_END,
		KEYWORD,
      LPARAN,
      RPARAN,
      QUOTE,
      STRING,
		UNKNOWN
   };

   Type type;
   std::string value;
   int numCharacters = 0;
};