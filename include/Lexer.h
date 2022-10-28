#include "Token.h"

#include <unordered_map>
#include <list>

class Lexer
{
public:
   Lexer(const std::string& filePath);
   ~Lexer();

private:
   void createTokenMap();
   void lexFile();
   void getToken(Token& token, const Token::Type& prevTokenType);
   bool getKeyword(Token& token);

   std::unordered_map<char, Token::Type> m_tokenMap;
   std::string m_filePath;
   std::string m_fileContents;
   int m_fileIndex;
   std::list<Token> m_tokenList;
};

void Lexer::createTokenMap()
{
   m_tokenMap[' '] = Token::WHITESPACE;

   m_tokenMap['\n'] = Token::NEWLINE;

   for (char c = '0'; c<= '9'; c++)
   {
      m_tokenMap[c] = Token::NUMBER;
   }

   for (char c = 'a'; c <= 'z'; c++)
   {
      m_tokenMap[c] = Token::LETTER;
   }
   for (char c = 'A'; c <= 'Z'; c++)
   {
      m_tokenMap[c] = Token::LETTER;
   }

   m_tokenMap[';'] = Token::EXPR_END;

   m_tokenMap['('] = Token::LPARAN;
   m_tokenMap[')'] = Token::RPARAN;

   m_tokenMap['"'] = Token::QUOTE;

}