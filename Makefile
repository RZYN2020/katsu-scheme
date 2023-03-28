CXX = g++
CXXFLAGS = -std=c++11 -Wall -Wextra -Wpedantic

SRCS = lexer.cc parser.cc main.cc
OBJS = $(SRCS:.cc=.o)

katsu: $(OBJS)
	$(CXX) $(CXXFLAGS) $(OBJS) -o katsu

%.o: %.cc lexer.hh parser.hh
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -f $(OBJS) katsu