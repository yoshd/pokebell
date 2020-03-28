#include <iostream>

extern "C"
{
  struct TwoTouchStringResult
  {
    size_t len;
    const char **data;
  };
  TwoTouchStringResult convert_to_two_touch_string(const char *);
  const char *convert_from_two_touch_string(const char *);
}

int main()
{
  auto word = "ごくろうさん";
  auto results = convert_to_two_touch_string(word);
  for (int i = 0; i < results.len; ++i)
  {
    std::cout << word << ": " << results.data[i] << std::endl;
  }

  auto two_touch_input = "25042395133103";
  auto result = convert_from_two_touch_string(two_touch_input);
  std::cout << two_touch_input << ": " << result << std::endl;
}
