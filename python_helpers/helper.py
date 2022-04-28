#testing auth api using request
import requests 
import json
import time
import grpc
import auth_pb2_grpc
import auth_pb2
channel = grpc.insecure_channel('localhost:50051')
stub = auth_pb2_grpc.AuthServiceStub(channel)


url = "http://localhost:8080/api/auth/"

#sign up and login
def signup(first_name, last_name, email, password):
    payload = {'first_name': first_name, 'last_name': last_name, 'email': email, 'password': password}
    headers = {'Content-Type': 'application/json'}
    response = requests.request("POST", url + "sign_up", headers=headers, data=json.dumps(payload))
    print(response.text)
    return response.text

def signin(email, password):
    payload = {'email': email, 'password': password}
    headers = {'Content-Type': 'application/json'}
    response = requests.request("POST", url + "sign_in", headers=headers, data=json.dumps(payload))
    print(response.text)
    return json.loads(response.text)

def signout(token, reftok):
    payload = {'refresh_token': reftok}
    headers = {'Content-Type' : 'application/json', 'Authorization': 'Bearer ' + token}
    response = requests.request("POST", url + "sign_out", headers=headers, data=json.dumps(payload))
    print(response.text)
    return response.text

def refresh(token, reftok):
    payload = {'refresh_token': reftok}
    headers = {'Content-Type' : 'application/json', 'Authorization': 'Bearer ' + token}
    response = requests.request("POST", url + "refresh", headers=headers, data=json.dumps(payload))
    print(response.text)
    return json.loads(response.text)

signup("test", "test", "test@test.com", "testtest")
tokens = signin("test@test.com", "testtest")
#sleep for 10 seconds
time.sleep(10)
tokens = refresh(tokens['access_token'], tokens['refresh_token'])
print(stub.GetUser(auth_pb2.JsonWebToken(jwt=tokens['access_token'])))
print(stub.HasAccess(auth_pb2.Resource(path='/api/auth/refresh', method='POST', jwt=tokens['access_token'])).has_access)
signout(tokens['access_token'], tokens['refresh_token'])
refresh(tokens['access_token'], tokens['refresh_token'])
