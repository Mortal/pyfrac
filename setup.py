# setup.py based on https://github.com/getsentry/milksnake
from setuptools import setup


def build_native(spec):
    build = spec.add_external_build(
        cmd=['cargo', 'build', '--release'],
    )

    spec.add_cffi_module(
        module_path='pyfrac._native',
        dylib=lambda: build.find_dylib('pyfrac', in_path='target/release'),
        header_filename=lambda: build.find_header('pyfrac.h', in_path='include'),
    )


setup(
    name='pyfrac',
    version='0.1.0',
    packages=['pyfrac'],
    author='Mathias Rav',
    license='GPL3+',
    author_email='m@git.strova.dk',
    description='Arbitrary precision Python REPL',
    # long_description=readme,
    include_package_data=True,
    zip_safe=False,
    platforms='any',
    install_requires=['milksnake'],
    setup_requires=['milksnake'],
    milksnake_tasks=[
        build_native,
    ],
)
