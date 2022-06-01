FROM python:3.8

RUN apt-get update
RUN apt-get install -y vim less python-dev libhdf5-dev

# # Install python env
RUN pip install --upgrade pip setuptools pipenv numpy
RUN pip install --no-binary=h5py h5py