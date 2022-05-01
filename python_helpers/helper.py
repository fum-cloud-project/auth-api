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

def update_me(token, first_name, last_name, email, password, url="http://localhost:8080/api/users/"):
    payload = {'first_name': first_name, 'last_name': last_name, 'email': email, 'password': password}
    headers = {'Content-Type': 'application/json', 'Authorization': 'Bearer ' + token}
    response = requests.request("PUT", url, headers=headers, data=json.dumps(payload))
    print(response.text)
    return json.loads(response.text)

def update_admin(token, _id, first_name, last_name, email, password, access_level, url="http://localhost:8080/api/users/"):
    payload = {'first_name': first_name, 'last_name': last_name, 'email': email, 'password': password, 'access_level': access_level}
    headers = {'Content-Type': 'application/json', 'Authorization': 'Bearer ' + token}
    print(url + _id)
    response = requests.request("PUT", url + _id, headers=headers, data=json.dumps(payload))
    print(response.text)
    return json.loads(response.text)

def create_user(token, first_name, last_name, email, password, access_level, url="http://localhost:8080/api/users/"):
    payload = {'first_name': first_name, 'last_name': last_name, 'email': email, 'password': password, 'access_level': access_level}
    headers = {'Content-Type': 'application/json', 'Authorization': 'Bearer ' + token}
    response = requests.request("POST", url, headers=headers, data=json.dumps(payload))
    print(response.text)
    return json.loads(response.text)

def delete_me(token, url="http://localhost:8080/api/users/"):
    headers = {'Content-Type': 'application/json', 'Authorization': 'Bearer ' + token}
    response = requests.request("DELETE", url, headers=headers)
    print(response.text)
    return response.text

def delete_admin(token, _id, url="http://localhost:8080/api/users/"):
    headers = {'Content-Type': 'application/json', 'Authorization': 'Bearer ' + token}
    response = requests.request("DELETE", url + _id, headers=headers)
    print(response.text)
    return response.text

def get_one_user(token, _id, url="http://localhost:8080/api/users/"):
    headers = {'Content-Type': 'application/json', 'Authorization': 'Bearer ' + token}
    response = requests.request("GET", url + _id, headers=headers)
    print(response.text)
    return json.loads(response.text)

def get_many_users(token, first_name, last_name, email, access_level, access_level_cmp, page =1, size=20, url="http://localhost:8080/api/users/"):
    headers = {'Content-Type': 'application/json', 'Authorization': 'Bearer ' + token}
    #build query string from parameters if not empty or None
    query_string = "?"
    if first_name is not None and first_name != "":
        query_string += "first_name=" + first_name + "&"
    if last_name is not None and last_name != "":
        query_string += "last_name=" + last_name + "&"
    if email is not None and email != "":
        query_string += "email=" + email + "&"
    if access_level is not None:
        query_string += "access_level=" + str(access_level) + "&"
    if access_level_cmp is not None:
        query_string += "access_level_cmp=" + str(access_level_cmp) + "&"
    
    #remove last &
    if query_string != "?":
        query_string = query_string[:-1]
    if query_string == "?":
        query_string = ""
    response = requests.request("GET", url + query_string, headers=headers)
    print(response.text)
    return json.loads(response.text)
# signup("test", "test", "test@test.com", "testtest")
# tokens = signin("test@test.com", "testtest")
# #sleep for 10 seconds
# time.sleep(10)
# tokens = refresh(tokens['access_token'], tokens['refresh_token'])
# print(stub.GetUser(auth_pb2.JsonWebToken(jwt=tokens['access_token'])))
# print(stub.HasAccess(auth_pb2.Resource(path='/api/auth/refresh', method='POST', jwt=tokens['access_token'])).has_access)
# signout(tokens['access_token'], tokens['refresh_token'])
# refresh(tokens['access_token'], tokens['refresh_token'])

#create 10 random users with access_level 1
tokens = signin("test@test.com", "testtest")

for i in range(10):
    create_user(tokens['access_token'], "test" + str(i), "test" + str(i), "test" + str(i) + "@test.com", "testtest", 1)
    tokens = refresh(tokens['access_token'], tokens['refresh_token'])

users = get_many_users(tokens['access_token'], "", "", "", 1, 0, 1, 10)
print(json.dumps(users, indent=4))
tokens = refresh(tokens['access_token'], tokens['refresh_token'])
#update created users
for i in range(10):
    update_admin(tokens['access_token'], users["data"]["data"][i]["_id"]["$oid"], "test" + str(i * 2), "test" + str(i * 2), "test" + str(i + 100) + "@test.com", "testtest", 1)
    tokens = refresh(tokens['access_token'], tokens['refresh_token'])

#delete created users
users = get_many_users(tokens['access_token'], "", "", "", 1, 0, 1, 10)
tokens = refresh(tokens['access_token'], tokens['refresh_token'])
#test getting one user
get_one_user(tokens['access_token'], users["data"]["data"][0]["_id"]["$oid"])
tokens = refresh(tokens['access_token'], tokens['refresh_token'])
#test deleting one user
delete_admin(tokens['access_token'], users["data"]["data"][0]["_id"]["$oid"])
tokens = refresh(tokens['access_token'], tokens['refresh_token'])
#test getting all users
users = get_many_users(tokens['access_token'], "", "", "", 1, 0, 1, 10)
print(json.dumps(users, indent=4))
tokens = refresh(tokens['access_token'], tokens['refresh_token'])
for user in users["data"]["data"]:
    delete_admin(tokens['access_token'], user["_id"]["$oid"])
    tokens = refresh(tokens['access_token'], tokens['refresh_token'])

signout(tokens['access_token'], tokens['refresh_token'])