#include <iostream>

// Rust側に公開するC++関数
// マングル回避のためextern "C"を付ける必要がある
extern "C" int hello_world_cpp()
{
    std::cout << "hello world from cpp!!" << std::endl;
    return 999;
}