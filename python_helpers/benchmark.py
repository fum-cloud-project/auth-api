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
    return response.text

def signin(email, password):
    payload = {'email': email, 'password': password}
    headers = {'Content-Type': 'application/json'}
    response = requests.request("POST", url + "sign_in", headers=headers, data=json.dumps(payload))
    return json.loads(response.text)

def signout(token, reftok):
    payload = {'refresh_token': reftok}
    headers = {'Content-Type' : 'application/json', 'Authorization': 'Bearer ' + token}
    response = requests.request("POST", url + "sign_out", headers=headers, data=json.dumps(payload))
    return response.text

def refresh(token, reftok):
    payload = {'refresh_token': reftok}
    headers = {'Content-Type' : 'application/json', 'Authorization': 'Bearer ' + token}
    response = requests.request("POST", url + "refresh", headers=headers, data=json.dumps(payload))
    return json.loads(response.text)


#create 1000 threads and test the performance
import threading
def worker(i):
    signup("test" + str(i), "test" + str(i), "test" + str(i) + "@gmail.com", "testtest" + str(i))
    tokens = signin("test" + str(i) + "@gmail.com", "testtest" + str(i))
    stub.GetUser(auth_pb2.JsonWebToken(jwt=tokens['access_token']))
    stub.HasAccess(auth_pb2.Resource(path='/api/auth/sign_out', method='POST', jwt=tokens['access_token'])).has_access
    tokens = refresh(tokens['access_token'], tokens['refresh_token'])
    signout(tokens['access_token'], tokens['refresh_token'])




def benchmark():
    threads = []
    now = time.time()
    for i in range(1000):
        t = threading.Thread(target=worker, args=(i,))
        threads.append(t)
        t.start()
    for t in threads:
        t.join()
    return (time.time() - now)

print(benchmark())
