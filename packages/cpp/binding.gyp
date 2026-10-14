{
  "targets": [
    {
      "target_name": "hello",
      "conditions": [
        ['OS=="win"', {
          "cflags": ["/std:c++20"]
          }],
        ['OS=="linux"', {
          "cflags": ["-std=c++20"]
          }]          
      ],
      "sources": [ "src/hello.cpp" ]
    }
  ]
}