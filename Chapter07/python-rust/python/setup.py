#!/usr/bin/env python
# -*- coding: utf-8 -*-

# Courtesy of https://github.com/kennethreitz/setup.py 

from setuptools import find_packages, setup, Command

# Package meta-data.
NAME = 'digest'
DESCRIPTION = 'A simple Python package that loads and executes a Rust function.'
URL = 'https://blog.x5ff.xyz'
AUTHOR = 'Claus Matzinger'
REQUIRES_PYTHON = '>=3.7.0'
VERSION = '0.1.0'
LICENSE = 'MIT'

setup(
    # Meta stuff
    name=NAME,
    version=VERSION,
    description=DESCRIPTION,
    long_description=DESCRIPTION,
    long_description_content_type='text/markdown',
    # ---
    package_dir={'':'src'}, # Declare src as root folder
    packages=find_packages(exclude=["tests", "*.tests", "*.tests.*", "tests.*"]), # Auto discover any Python packages
    python_requires=REQUIRES_PYTHON,
    # Scripts that will be generated invoke this method
    entry_points={
        'setuptools.installation': ['eggsecutable=digest:main'],
    },
    include_package_data=True,
    license=LICENSE,
    classifiers=[
        # Trove classifiers
        # Full list: https://pypi.python.org/pypi?%3Aaction=list_classifiers
        'License :: OSI Approved :: MIT License',
        'Programming Language :: Python',
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.7',
        'Programming Language :: Python :: Implementation :: CPython',
        'Programming Language :: Python :: Implementation :: PyPy'
    ],
)
