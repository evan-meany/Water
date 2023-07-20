#include "../include/Lexer.h"

#include <iostream>
#include <windows.h>
#include <fstream>
#include <sstream>

static void printFilesInDirectory(const std::string& directoryPath) 
{
   WIN32_FIND_DATAA findFileData;
   HANDLE hFind = FindFirstFileA((directoryPath + "\\*").c_str(), &findFileData);

   if (hFind != INVALID_HANDLE_VALUE) 
   {
      do 
      {
         if (!(findFileData.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY)) 
         {
            std::cout << findFileData.cFileName << std::endl;
         }
      } while (FindNextFileA(hFind, &findFileData) != 0);

      FindClose(hFind);
   }
}

static bool readFile(const std::string& filePath, std::string& fileContents)
{
   std::fstream file;
   char buffer[MAX_PATH];
   GetCurrentDirectoryA(MAX_PATH, buffer);
   printFilesInDirectory(".");
   std::cout << "Current Working Directory: " << buffer << std::endl;
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
      // Check if the fail bit is set (indicating a general I/O error).
      std::cerr << "Failed to open the file: " << filePath << std::endl;
      std::cerr << "Error code: " << errno << std::endl;

      // Get a descriptive error message based on the errno (on Windows).
      char errorMessage[256];
      if (strerror_s(errorMessage, sizeof(errorMessage), errno) == 0)
      {
         std::cerr << "Error message: " << errorMessage << std::endl;
      }

      return false;
   }

   return true;
}

static std::string getTokenTypeName(const int& type)
{
   std::string typeString;
   switch (type)
   {
      case Token::WHITESPACE:
         typeString = "WHITESPACE";
         break;
      case Token::NEWLINE:
         typeString = "NEWLINE";
         break;
      case Token::TERMINATOR:
         typeString = "TERMINATOR";
         break;
      case Token::NUMERIC:
         typeString = "NUMERIC";
         break;
      case Token::STRING:
         typeString = "STRING";
         break;
      case Token::KEYWORD:
         typeString = "KEYWORD";
         break;
      case Token::IDENTIFIER:
         typeString = "IDENTIFIER";
         break;
      case Token::OPERATOR:
         typeString = "OPERATOR";
         break;
      case Token::PUNCTUATION:
         typeString = "PUNCTUATION";
         break;
      case Token::SCOPE:
         typeString = "SCOPE";
         break;
      case Token::UNKNOWN:
         typeString = "UNKNOWN";
         break;
   }

   return typeString;
}

static std::string getTokenValueName(const Token& token)
{
   if (token.type != Token::WHITESPACE && token.type != Token::NEWLINE)
   {
      return token.value;
   }

   std::string valuePrintString;

   for (int i = 0; i < token.value.length(); i++)
   {
      switch (token.value[i])
      {
         case '\t':
            valuePrintString += '\\';
            valuePrintString += 't';
            break;
         case '\n':
            valuePrintString += '\\';
            valuePrintString += 'n';
            break;
         case '\r':
            valuePrintString += '\\';
            valuePrintString += 'r';
            break;
         default:
            valuePrintString += token.value[i];
            break;
      }
   }

   return valuePrintString;
}

Lexer::Lexer(const std::string& filePath)
{
   createTokenMap();
   m_filePath = filePath;
   lexFile();
}

Lexer::~Lexer()
{}

void Lexer::lexFile()
{
   if (!readFile(m_filePath, m_fileContents)) { return; }
   
   m_fileIndex = 0;
   m_fileSize = m_fileContents.size();

   int id = 0;
   CharacterType prevCharType = WHITESPACE;
   while (m_fileIndex < m_fileSize)
   {
      Token token;
      getToken(token, prevCharType);

      if (token.type == Token::IDENTIFIER)
      {
         checkForKeyword(token);
      }

      token.id = id++;
      m_tokenList.push_back(token);
   }
}

void Lexer::getToken(Token& token, CharacterType& prevCharType)
{
   if (m_fileIndex >= m_fileSize)
   {
      return;
   }

   char c = m_fileContents[m_fileIndex];
   switch (m_charTypeMap[c])
   {
      case WHITESPACE:
         if (token.numCharacters == 0 || prevCharType == WHITESPACE)
         {
            token.type = Token::WHITESPACE;
            token.value += c;
            token.numCharacters++;
            m_fileIndex++;
            prevCharType = WHITESPACE;  
            getToken(token, prevCharType);
         }

         break;
      
      case NEWLINE:
         if (token.numCharacters == 0 || prevCharType == NEWLINE)
         {
            token.type = Token::NEWLINE;
            token.value += c;
            token.numCharacters++;
            m_fileIndex++;
            prevCharType = NEWLINE;
            getToken(token, prevCharType);
         }

         break;

      case NUMBER:
         if (token.numCharacters == 0 || prevCharType == NUMBER)
         {
            token.type = Token::NUMERIC;
            token.value += c;
            token.numCharacters++;
            m_fileIndex++;
            prevCharType = NUMBER;
            getToken(token, prevCharType);
         }

         break;

      case LETTER:
         if (token.numCharacters == 0 || prevCharType == LETTER || prevCharType == QUOTE)
         {
            if (token.type == Token::STRING || prevCharType == QUOTE)
            {
               token.type = Token::STRING;
            }
            else
            {
               token.type = Token::IDENTIFIER;
            }
            token.value += c;
            token.numCharacters++;
            m_fileIndex++;
            prevCharType = LETTER;
            getToken(token, prevCharType);
         }

         break;

      case QUOTE:
         if (token.numCharacters == 0)
         {
            token.type = Token::STRING;
            token.value += c;
            token.numCharacters++;
            m_fileIndex++;
            prevCharType = QUOTE;
            getToken(token, prevCharType);
            if (prevCharType != LETTER && prevCharType != QUOTE)
            {
               token.type = Token::UNKNOWN;
            }         
         }
         else if (prevCharType == LETTER || prevCharType == QUOTE) 
         {
            token.type = Token::STRING;
            token.value += c;
            token.numCharacters++;
            m_fileIndex++;
            prevCharType = QUOTE;
         }

         break;

      case SEMICOLON:
         if (token.numCharacters == 0)
         {  
            token.type = Token::TERMINATOR;
            token.value += c;
            token.numCharacters++;
            m_fileIndex++;
            prevCharType = SEMICOLON;
         }

         break;

      case L_PARAN:
      case R_PARAN:
         if (token.numCharacters == 0)
         {  
            token.type = Token::PUNCTUATION;
            token.value += c;
            token.numCharacters++;
            m_fileIndex++;
            prevCharType = m_charTypeMap[c];
         }

         break;

      case L_CBRCKT:
      case R_CBRCKT:
         if (token.numCharacters == 0)
         {  
            token.type = Token::SCOPE;
            token.value += c;
            token.numCharacters++;
            m_fileIndex++;
            prevCharType = m_charTypeMap[c];
         }

         break;

      case OPERATOR:
         if (token.numCharacters == 0)
         {
            token.type = Token::OPERATOR;
            token.value += c;
            token.numCharacters++;
            m_fileIndex++;
            prevCharType = m_charTypeMap[c];
         }

         break;

      case PUNCTUATION:
         if (token.numCharacters == 0)
         {
            token.type = Token::PUNCTUATION;
            token.value += c;
            token.numCharacters++;
            m_fileIndex++;
            prevCharType = m_charTypeMap[c];
         }

         break;

      case UNKNOWN:
         if (token.numCharacters == 0)
         {
            token.type = Token::UNKNOWN;
            token.value += c;
            token.numCharacters++;
            m_fileIndex++;
            prevCharType = UNKNOWN;
         }
         break;         
   }


   return;
}

void Lexer::checkForKeyword(Token& token)
{
   if (token.value == "if" || token.value == "else" || token.value == "while" || token.value == "print")
   {
      token.type = Token::KEYWORD;
   }
}

void Lexer::printTokenList()
{
   for (int i = 0; i < m_tokenList.size(); i++)
   {
      std::cout << "ID: " << m_tokenList[i].id;
      std::cout << ", TYPE: " << getTokenTypeName(m_tokenList[i].type);
      std::cout << ", VALUE: \'" << getTokenValueName(m_tokenList[i]) << "\'";
      std::cout << ", SIZE: " << m_tokenList[i].numCharacters << std::endl;
   }
}

void Lexer::createTokenMap()
{
   m_charTypeMap[' '] = WHITESPACE;
   m_charTypeMap['\t'] = WHITESPACE;

   m_charTypeMap['\n'] = NEWLINE;
   m_charTypeMap['\r'] = NEWLINE;

   for (char c = '0'; c <= '9'; c++)
   {
      m_charTypeMap[c] = NUMBER;
   }

   for (char c = 'a'; c <= 'z'; c++)
   {
      m_charTypeMap[c] = LETTER;
   }
   for (char c = 'A'; c <= 'Z'; c++)
   {
      m_charTypeMap[c] = LETTER;
   }
   m_charTypeMap['_'] = LETTER;

   m_charTypeMap['"'] = QUOTE;

   m_charTypeMap[';'] = SEMICOLON;

   m_charTypeMap['('] = L_PARAN;
   m_charTypeMap[')'] = R_PARAN;

   m_charTypeMap['{'] = L_CBRCKT;
   m_charTypeMap['}'] = R_CBRCKT;

   m_charTypeMap['*'] = OPERATOR;
   m_charTypeMap['-'] = OPERATOR;
   m_charTypeMap['+'] = OPERATOR;
   m_charTypeMap['='] = OPERATOR;
   m_charTypeMap['<'] = OPERATOR;
   m_charTypeMap['>'] = OPERATOR;
   m_charTypeMap['|'] = OPERATOR;
   m_charTypeMap['!'] = OPERATOR;
   m_charTypeMap['&'] = OPERATOR;

   m_charTypeMap[','] = PUNCTUATION;

   // Unhandled characters
   m_charTypeMap['#'] = UNKNOWN;
   m_charTypeMap['$'] = UNKNOWN;
   m_charTypeMap['%'] = UNKNOWN;
   m_charTypeMap['\''] = UNKNOWN;
   m_charTypeMap['.'] = UNKNOWN;
   m_charTypeMap['\\'] = UNKNOWN;
   m_charTypeMap['/'] = UNKNOWN;
   m_charTypeMap['?'] = UNKNOWN;
   m_charTypeMap['`'] = UNKNOWN;
   m_charTypeMap['~'] = UNKNOWN;
   m_charTypeMap['^'] = UNKNOWN;
   m_charTypeMap['['] = UNKNOWN;
   m_charTypeMap[']'] = UNKNOWN;

}