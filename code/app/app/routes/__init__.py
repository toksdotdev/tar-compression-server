from app import app
import requests
import json
from flask import Response, send_file, make_response, request
from werkzeug.utils import secure_filename
import os
import json


@app.route('/upload', methods=['POST'])
def uploadMedia():
    if 'file' not in request.files:
        return json.dumps({'message': 'A file must be specified'})
    else:
        submitted_file = request.files['file']
        data = {"file_id": submitted_file.filename}
        files = {"file": submitted_file}

        response = requests.post(
            'http://localhost:8000/upload',
            files=files,
            data=data
        )

        return Response(response.content, response.status_code, content_type='application/json')


@app.route('/file/<fileId>')
def hello_name(fileId):
    response = requests.get(
        'http://localhost:8000/file/{0}'.format(fileId.strip()),
        stream=True
    )

    print(response.status_code)

    if response.status_code != 200:
        print(str(response.content))

        return str(response.content), 200
        # return json.dumps({'status': False, 'message': response.text})

    return Response(
        response.iter_content(chunk_size=10*1024),
        mimetype='application/x-tar',
        content_type='application/x-tar'
    )
