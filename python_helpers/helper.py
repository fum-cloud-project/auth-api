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
    print(json.dumps(payload, indent=4, ensure_ascii=False))
    response = requests.request("POST", url + "sign_up", headers=headers, data=json.dumps(payload))
    print(json.dumps(json.loads(response.text), indent=4))
    return response.text

def signin(email, password):
    payload = {'email': email, 'password': password}
    headers = {'Content-Type': 'application/json'}
    print(json.dumps(payload, indent=4, ensure_ascii=False))
    response = requests.request("POST", url + "sign_in", headers=headers, data=json.dumps(payload))
    print(json.dumps(json.loads(response.text), indent=4))
    return json.loads(response.text)

def signout(token, reftok):
    payload = {'refresh_token': reftok}
    headers = {'Content-Type' : 'application/json', 'Authorization': 'Bearer ' + token}
    print(json.dumps(payload, indent=4, ensure_ascii=False))
    response = requests.request("POST", url + "sign_out", headers=headers, data=json.dumps(payload))
    print(json.dumps(json.loads(response.text), indent=4))
    return response.text

def refresh(token, reftok):
    payload = {'refresh_token': reftok}
    headers = {'Content-Type' : 'application/json', 'Authorization': 'Bearer ' + token}
    print(json.dumps(payload, indent=4, ensure_ascii=False))
    response = requests.request("POST", url + "refresh", headers=headers, data=json.dumps(payload))
    print(json.dumps(json.loads(response.text), indent=4))
    return json.loads(response.text)

def update_me(token, first_name, last_name, email, password, url="http://localhost:8080/api/users/"):
    payload = {'first_name': first_name, 'last_name': last_name, 'email': email, 'password': password}
    print(json.dumps(payload, indent=4, ensure_ascii=False))
    headers = {'Content-Type': 'application/json', 'Authorization': 'Bearer ' + token}
    response = requests.request("PUT", url, headers=headers, data=json.dumps(payload))
    print(json.dumps(json.loads(response.text), indent=4))
    return json.loads(response.text)

def update_admin(token, _id, first_name, last_name, email, password, access_level, url="http://localhost:8080/api/users/"):
    payload = {'first_name': first_name, 'last_name': last_name, 'email': email, 'password': password, 'access_level': access_level}
    headers = {'Content-Type': 'application/json', 'Authorization': 'Bearer ' + token}
    print(json.dumps(payload, indent=4, ensure_ascii=False))
    response = requests.request("PUT", url + _id, headers=headers, data=json.dumps(payload))
    print(json.dumps(json.loads(response.text), indent=4))
    return json.loads(response.text)

def create_user(token, first_name, last_name, email, password, access_level, url="http://localhost:8080/api/users/"):
    payload = {'first_name': first_name, 'last_name': last_name, 'email': email, 'password': password, 'access_level': access_level}
    headers = {'Content-Type': 'application/json', 'Authorization': 'Bearer ' + token}
    print(json.dumps(payload, indent=4, ensure_ascii=False))
    response = requests.request("POST", url, headers=headers, data=json.dumps(payload))
    print(json.dumps(json.loads(response.text), indent=4))
    return json.loads(response.text)

def delete_me(token, url="http://localhost:8080/api/users/"):
    headers = {'Content-Type': 'application/json', 'Authorization': 'Bearer ' + token}
    response = requests.request("DELETE", url, headers=headers)
    print(json.dumps(json.loads(response.text), indent=4))
    return response.text

def delete_admin(token, _id, url="http://localhost:8080/api/users/"):
    headers = {'Content-Type': 'application/json', 'Authorization': 'Bearer ' + token}
    response = requests.request("DELETE", url + _id, headers=headers)
    print(json.dumps(json.loads(response.text), indent=4))
    return response.text

def get_one_user(token, _id, url="http://localhost:8080/api/users/"):
    headers = {'Content-Type': 'application/json', 'Authorization': 'Bearer ' + token}
    response = requests.request("GET", url + _id, headers=headers)
    print(json.dumps(json.loads(response.text), indent=4))
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
    print(query_string)
    print(json.dumps(json.loads(response.text), indent=4))
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

# create 10 random users with access_level 1
tokens = signin("test@test.com", "testtest")

for i in range(10):
    create_user(tokens['access_token'], "test" + str(i), "test" + str(i), "test" + str(i) + "@test.com", "testtest", 1)
    tokens = refresh(tokens['access_token'], tokens['refresh_token'])

users = get_many_users(tokens['access_token'], "", "", "", 1, 0, 1, 10)
print(json.dumps(users, indent=4))
tokens = refresh(tokens['access_token'], tokens['refresh_token'])
#update created users
for i in range(10):
    update_admin(tokens['access_token'], users["data"]["data"][i]["_id"]["$oid"], "test" + str(i * 2), "test" + str(i * 2), "test" + str(i + 100) + "@test.com", "testtest", access_level=2000)
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


#create a user and sign in with it then delete the user and test login tokens they should fail
print("\n\n\n")
print("create a user and sign in with it then delete the user and test login tokens they should fail")
print("creating user")
create_user(tokens['access_token'], "test", "test", "testaccdelete@delete.com", "testtest", 1)
print("user created")
print("signing in")
user_tokens = signin("testaccdelete@delete.com", "testtest")
print("user signed in")
tokens = refresh(tokens['access_token'], tokens['refresh_token'])
print("retrieving user")
user_acc = get_many_users(tokens['access_token'], "", "", "", 1, 0, 1, 10)
print("user retrieved")
tokens = refresh(tokens['access_token'], tokens['refresh_token'])
print("deleting user")
delete_admin(tokens['access_token'], user_acc["data"]["data"][0]["_id"]["$oid"])
print("user deleted")
tokens = refresh(tokens['access_token'], tokens['refresh_token'])
delete_admin(tokens['access_token'], user_acc["data"]["data"][0]["_id"]["$oid"])
tokens = refresh(tokens['access_token'], tokens['refresh_token'])
get_one_user(tokens['access_token'], user_acc["data"]["data"][0]["_id"]["$oid"])
tokens = refresh(tokens['access_token'], tokens['refresh_token'])
print("test delete")
update_me(user_tokens['access_token'], "test", "test", "", "")


signout(tokens['access_token'], tokens['refresh_token'])


# signup("test", "test", "test@test.com", "testtest")
# signup("test", "test", "test_badmail.com", "testtest")
# signup("test", "test", "testtesttest@test.com", "shortp")
# signup("test", "test", "testttseqwettt@test.com", "testtest")


# signin("nono@nono.com", "nonononononono")
# tokens = signin("test@test.com", "testtest")

# get_many_users(tokens['access_token'], "", "", "", 1, 0, 1, 10)

# tokens = refresh(tokens['access_token'], tokens['refresh_token'])

# signout(tokens['access_token'], tokens['refresh_token'])
# signout(tokens['access_token'], tokens['refresh_token'])


