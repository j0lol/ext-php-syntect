<?php

$making_tests = false;

if ($making_tests) {
    // Generate test files
    file_put_contents("test/test.html", syntect_highlight("# Hello", "md"));
    file_put_contents("test/test.css", syntect_css("./test/Catppuccin Mocha.tmTheme"));
} else {
    // Check test files
    assert(file_get_contents("test/test.html") == syntect_highlight("# Hello", "md"));
    assert(file_get_contents("test/test.css") == syntect_css("test/Catppuccin Mocha.tmTheme"));
}
