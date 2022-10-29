#include <string>

struct Token
{
   enum Type
   {
		WHITESPACE,
      NEWLINE,
		TERMINATOR,
      NUMERIC,
      STRING,
		KEYWORD,
      IDENTIFIER,
      OPERATOR,
      PUNCTUATION,
      SCOPE,
		UNKNOWN
   };

   int id;
   Type type;
   std::string value;
   int numCharacters = 0;
};