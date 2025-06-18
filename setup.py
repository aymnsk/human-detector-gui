from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="human-detector",
    version="0.1.0",
    rust_extensions=[RustExtension("human_detector.human_detector", binding=Binding.PyO3)],
    packages=["human_detector"],
    zip_safe=False,
)
