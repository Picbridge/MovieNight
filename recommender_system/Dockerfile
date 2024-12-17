# Use an official Python runtime as a parent image
FROM python:3.9
ENV PYTHONPATH=/app

# Set the working directory
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . .

#Copy script files only
#---------------------------------------------------------------
# Only copy the necessary files
# COPY app /app/app
# COPY run.py /app/
# COPY requirements.txt /app/

# Copy the requirements.txt file separately (if it exists in the same directory)
#---------------------------------------------------------------

# Install the dependencies
RUN pip install -r requirements.txt

# Define environment variable
ENV FLASK_APP=run.py
ENV FLASK_RUN_HOST=0.0.0.0

# Make port 5000 available to the world outside this container
EXPOSE 5000

# Run the Flask app
CMD ["flask", "run"]
