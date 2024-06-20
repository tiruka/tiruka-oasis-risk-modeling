#!/usr/bin/env python
from setuptools import dist

dist.Distribution().fetch_build_eggs(["setuptools_rust"])
from setuptools import setup
from setuptools_rust import Binding, RustExtension


setup(
    name="tiruka-oasis-risk-modeling",
    version="0.1",
    rust_extensions=[
        RustExtension(
            ".tiruka_oasis_risk_modeling.tiruka_oasis_risk_modeling",
            path="Cargo.toml",
            binding=Binding.PyO3,
        )
    ],
    packages=["tiruka_oasis_risk_modeling"],
    include_package_data=True,
    package_data={"": ["*.csv"]},
    zip_safe=False,
)
