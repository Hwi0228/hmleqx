import unittest

import hmleqx


class HmleqxPythonApiTests(unittest.TestCase):
    def test_to_latex_uses_native_rust_parser(self):
        self.assertEqual(hmleqx.to_latex("1 over 2"), r"\dfrac{1}{2}")

    def test_convert_and_eq_to_latex_are_aliases(self):
        source = "sin x"
        expected = r"\sin x"
        self.assertEqual(hmleqx.convert(source), expected)
        self.assertEqual(hmleqx.eq_to_latex(source), expected)

    def test_parse_returns_python_ast_objects(self):
        ast = hmleqx.parse("1 over pi")
        self.assertEqual(
            ast,
            {
                "Frac": {
                    "num": {"Number": "1"},
                    "den": {"Symbol": "pi"},
                    "bar": True,
                }
            },
        )

    def test_version_is_available(self):
        self.assertEqual(hmleqx.__version__, "0.2.1")
        self.assertEqual(hmleqx.version(), hmleqx.__version__)

    def test_non_string_input_raises_type_error(self):
        with self.assertRaises(TypeError):
            hmleqx.to_latex(None)


if __name__ == "__main__":
    unittest.main()
