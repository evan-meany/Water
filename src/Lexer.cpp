#include "../include/Lexer.h"

#include <iostream>
#include <fstream>
#include <sstream>

static void readFile(const std::string& filePath, std::string& fileContents)
{
   std::fstream file;

   file.open(filePath);

   if (file.is_open())
   {
      std::stringstream ss;
      ss << file .rdbuf();
      fileContents = ss.str();
      file.close();
   }
   else
   {
      throw "could not open: " + filePath;
   }
}


Lexer::Lexer(const std::string& filePath)
{
   m_filePath = filePath;
   lexFile();
}

Lexer::~Lexer()
{}

void Lexer::lexFile()
{
   readFile(m_filePath, m_fileContents);
   
   m_fileIndex = 0;

   Token::Type prevTokenType;
   while (m_fileIndex < m_fileContents.size())
   {
      Token token;
      getToken(token, prevTokenType);
      m_tokenList.push_back(token);

      if (token.type == Token::LETTER)
      {
         if (!getKeyword(token))
         {
            std::cout << "Bad syntax" << std::endl;
         }
      }
      prevTokenType = token.type;
   }
}

void Lexer::getToken(Token& token, const Token::Type& prevTokenType)
{
   char c = m_fileContents[m_fileIndex];
   switch (m_tokenMap[c])
   {
      case Token::WHITESPACE:
         if (token.numCharacters == 0 || prevTokenType == Token::WHITESPACE)
         {
            token.type = Token::WHITESPACE;
            token.value += c;
            token.numCharacters++;
            m_fileIndex++;             
            getToken(token, token.type);
         }

         break;
      
      case Token::NEWLINE:
         if (token.numCharacters == 0 || prevTokenType == Token::NEWLINE)
         {
            token.type = Token::NEWLINE;
            token.value += c;
            token.numCharacters++;
            m_fileIndex++;
            getToken(token, token.type);
         }

         break;

      case Token::NUMBER:
         if (token.numCharacters == 0 || prevTokenType == Token::NUMBER)
         {
            token.type = Token::NUMBER;
            token.value += c;
            token.numCharacters++;
            m_fileIndex++;
            getToken(token, token.type);
         }

         break;

      case Token::LETTER:
         if (token.numCharacters == 0 || prevTokenType == Token::LETTER || prevTokenType == Token::STRING || prevTokenType == Token::QUOTE)
         {
            if (prevTokenType == Token::STRING || prevTokenType == Token::QUOTE)
            {
               token.type = Token::STRING;
            }
            else
            {
               token.type = Token::LETTER;
            }
            token.value += c;
            token.numCharacters++;
            m_fileIndex++;
            getToken(token, token.type);

         }

         break;

      case Token::EXPR_END:
         break;
      case Token::LPARAN:
         break;
      case Token::RPARAN:
         break;
      case Token::QUOTE:
         break;
      case Token::UNKNOWN:
         break;         
   }


   return;
}

bool Lexer::getKeyword(Token& token)
{
   if (token.value == "if" || token.value == "else" || token.value == "while" || token.value == "print")
   {
      token.type = Token::KEYWORD;
      return true;
   }
   else
   {
      return false;
   }
}