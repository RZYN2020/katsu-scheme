#include <string>
#include <vector>

enum TokenType {
    RED,
    GREEN,
    BLUE
};

class Token {
private:
    TokenType type;
    std::string value;
public:
    Token(){}
    Token(TokenType type, std::string value): type(type), value(value) {}
    TokenType getType() { return type; }
    std::string getValue() { return value; }
};

class Lexer {
private:
    std::string input;
    int position;
    char currentChar;

public:
    // class Iterator {
    // private:
    //     Lexer& lexer;
    //     int position;
    // public:
    //     Iterator(Lexer& lexer, int position): lexer(lexer), position(position) {}
    //     Token* operator*() { return lexer.getNextToken(); } // dereference operator
    //     Iterator& operator++() { lexer.getNextToken(); return *this; } // pre-increment operator
    //     bool operator!=(const Iterator& other) { return position != other.position; } // inequality operator
    // };
    // Iterator begin() { return Iterator(*this, 0); }
    // Iterator end() { return Iterator(*this, input.size()); }

    Lexer(std::string input): input(input), position(0), currentChar(input[0]) {}
    Token* getNextToken();
};