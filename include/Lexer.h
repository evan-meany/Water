#include "Token.h"

#include <unordered_map>
#include <vector>

class Lexer
{
   enum CharacterType
   {
      WHITESPACE,
      NEWLINE,
      NUMBER,
      LETTER,
      QUOTE,
      SEMICOLON,
      L_PARAN,
      R_PARAN,
      L_CBRCKT,
      R_CBRCKT,
      OPERATOR,
      PUNCTUATION,
      UNKNOWN
   };

public:
   Lexer(const std::string& filePath);
   ~Lexer();
   void printTokenList();

private:
   void createTokenMap();
   void lexFile();
   void getToken(Token& token, CharacterType& prevTokenType);
   void checkForKeyword(Token& token);

   std::unordered_map<char, CharacterType> m_charTypeMap;
   std::string m_filePath;
   std::string m_fileContents;
   int m_fileIndex;
   size_t m_fileSize;
   std::vector<Token> m_tokenList;
};

