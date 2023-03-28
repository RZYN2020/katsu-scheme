#include <iostream>

#include <lexer.hh>
#include <parser.hh>

int main() {
    std::string input = "3 + 5 * 2";
    Lexer lexer(input);
    Token* token;
    while ((token = lexer.getNextToken())->getType() != EOF) {
        std::cout << token->getValue() << std::endl;
    }
}