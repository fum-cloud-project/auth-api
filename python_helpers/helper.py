# testing auth api using request
from __future__ import print_function, unicode_literals
from turtle import clear
from PyInquirer import prompt, Separator
from tabulate import tabulate
import os
import sys
import re

import requests
import json
import time
import grpc
import auth_pb2_grpc
import auth_pb2
import random
channel = grpc.insecure_channel('localhost:50051')
stub = auth_pb2_grpc.AuthServiceStub(channel)


url = "http://192.168.122.94:30900/api/auth/"
admin_email = "admin@mail.com"
admin_password = "adminadmin"
debug_global = False
# sign up and login


def signup(first_name, last_name, email, password):
    payload = {'first_name': first_name, 'last_name': last_name,
               'email': email, 'password': password}
    headers = {'Content-Type': 'application/json'}
    if debug_global:
        print(json.dumps(payload, indent=4, ensure_ascii=False))
    response = requests.request(
        "POST", url + "sign_up", headers=headers, data=json.dumps(payload))
    if debug_global:
        print(json.dumps(json.loads(response.text), indent=4))
    return json.loads(response.text), response.status_code


def signin(email, password):
    payload = {'email': email, 'password': password}
    headers = {'Content-Type': 'application/json'}
    if debug_global:
        print(json.dumps(payload, indent=4, ensure_ascii=False))
    response = requests.request(
        "POST", url + "sign_in", headers=headers, data=json.dumps(payload))
    if debug_global:
        print(json.dumps(json.loads(response.text), indent=4))
    return json.loads(response.text), response.status_code


def signout(token, reftok):
    payload = {'refresh_token': reftok}
    headers = {'Content-Type': 'application/json',
               'Authorization': 'Bearer ' + token}
    if debug_global:
        print(json.dumps(payload, indent=4, ensure_ascii=False))
    response = requests.request(
        "POST", url + "sign_out", headers=headers, data=json.dumps(payload))
    if debug_global:
        print(json.dumps(json.loads(response.text), indent=4))
    return json.loads(response.text), response.status_code


def refresh(token, reftok):
    payload = {'refresh_token': reftok}
    headers = {'Content-Type': 'application/json',
               'Authorization': 'Bearer ' + token}
    if debug_global:
        print(json.dumps(payload, indent=4, ensure_ascii=False))
    response = requests.request(
        "POST", url + "refresh", headers=headers, data=json.dumps(payload))
    if debug_global:
        print(json.dumps(json.loads(response.text), indent=4))
    return json.loads(response.text), response.status_code


def update_me(token, first_name, last_name, email, password, url="http://192.168.122.94:30900/api/users/"):
    # create payload based on what is passed in if they are not None or empty string
    payload = {}
    if first_name:
        payload['first_name'] = first_name
    if last_name:
        payload['last_name'] = last_name
    if email:
        payload['email'] = email
    if password:
        payload['password'] = password

    if debug_global:
        print(json.dumps(payload, indent=4, ensure_ascii=False))
    headers = {'Content-Type': 'application/json',
               'Authorization': 'Bearer ' + token}
    response = requests.request(
        "PUT", url, headers=headers, data=json.dumps(payload))
    if debug_global:
        print(json.dumps(json.loads(response.text), indent=4))
    return json.loads(response.text), response.status_code


def update_admin(token, _id, first_name, last_name, email, password, access_level, url="http://192.168.122.94:30900/api/users/"):
    payload = {}
    if first_name:
        payload['first_name'] = first_name
    if last_name:
        payload['last_name'] = last_name
    if email:
        payload['email'] = email
    if password:
        payload['password'] = password
    headers = {'Content-Type': 'application/json',
               'Authorization': 'Bearer ' + token}
    if debug_global:
        print(json.dumps(payload, indent=4, ensure_ascii=False))
    response = requests.request(
        "PUT", url + _id, headers=headers, data=json.dumps(payload))
    if debug_global:
        print(json.dumps(json.loads(response.text), indent=4))
    return json.loads(response.text), response.status_code


def create_user(token, first_name, last_name, email, password, access_level, url="http://192.168.122.94:30900/api/users/"):
    payload = {'first_name': first_name, 'last_name': last_name,
               'email': email, 'password': password, 'access_level': access_level}
    headers = {'Content-Type': 'application/json',
               'Authorization': 'Bearer ' + token}
    if debug_global:
        print(json.dumps(payload, indent=4, ensure_ascii=False))
    response = requests.request(
        "POST", url, headers=headers, data=json.dumps(payload))
    if debug_global:
        print(json.dumps(json.loads(response.text), indent=4))
    return json.loads(response.text), response.status_code


def delete_me(token, url="http://192.168.122.94:30900/api/users/"):
    headers = {'Content-Type': 'application/json',
               'Authorization': 'Bearer ' + token}
    response = requests.request("DELETE", url, headers=headers)
    if debug_global:
        print(json.dumps(json.loads(response.text), indent=4))
    return json.loads(response.text), response.status_code


def delete_admin(token, _id, url="http://192.168.122.94:30900/api/users/"):
    headers = {'Content-Type': 'application/json',
               'Authorization': 'Bearer ' + token}
    response = requests.request("DELETE", url + _id, headers=headers)
    if debug_global:
        print(json.dumps(json.loads(response.text), indent=4))
    return json.loads(response.text), response.status_code


def get_one_user(token, _id, url="http://192.168.122.94:30900/api/users/"):
    headers = {'Content-Type': 'application/json',
               'Authorization': 'Bearer ' + token}
    response = requests.request("GET", url + _id, headers=headers)
    if debug_global:
        print(json.dumps(json.loads(response.text), indent=4))
    return json.loads(response.text), response.status_code


def get_my_acc(token, url="http://192.168.122.94:30900/api/users/my_acc"):
    headers = {'Authorization': 'Bearer ' + token}
    response = requests.request("GET", url, headers=headers)
    if debug_global:
        print(json.dumps(json.loads(response.text), indent=4))
    return json.loads(response.text), response.status_code


def get_many_users(token, first_name, last_name, email, access_level, access_level_cmp, page=1, size=20, url="http://192.168.122.94:30900/api/users/"):
    headers = {'Content-Type': 'application/json',
               'Authorization': 'Bearer ' + token}
    # build query string from parameters if not empty or None
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

    # remove last &
    if query_string != "?":
        query_string = query_string[:-1]
    if query_string == "?":
        query_string = ""
    response = requests.request("GET", url + query_string, headers=headers)
    if debug_global:
        print(query_string)
    if debug_global:
        print(json.dumps(json.loads(response.text), indent=4))
    return json.loads(response.text), response.status_code


def automated_test_pipeline():
    # signup
    random_email_signup = "test" + str(random.randint(0, 100)) + "@test.com"
    _, code = signup("test", "test", random_email_signup, "testtest")
    if code != 200:
        print("TEST[signup with correct data] failed")
    else:
        print("TEST[signup with correct data] succeeded")
    _, code = signup("test", "test", "ttttt.com", "te")
    if code != 400:
        print("TEST[signup with incorrect email and password] failed")
    else:
        print("TEST[signup with incorrect email and password] succeeded")

    # signin
    _, code = signin(random_email_signup, "testtest")
    if code != 200:
        print("TEST[signin with correct data] failed")
    else:
        print("TEST[signin with correct data] succeeded")

    # signout
    tokens, code = signin(random_email_signup, "testtest")
    if code != 200:
        print("TEST[signout with correct data] failed")
    else:
        _, code = signout(tokens["access_token"], tokens["refresh_token"])
        if code != 200:
            print("TEST[signout with correct data] failed")
        else:
            print("TEST[signout with correct data] succeeded")

    # create user with random email
    tokens, code = signin(admin_email, admin_password)
    random_email = "test" + str(random.randint(0, 100)) + "@test.com"
    if code != 200:
        print("TEST[create user with correct data] failed")
    else:
        _, code = create_user(
            tokens["access_token"], "test", "test", random_email, "testtest", 1)
        if code != 200:
            print("TEST[create user with correct data] failed")
        else:
            print("TEST[create user with correct data] succeeded")

    # get many users
    tokens, code = refresh(tokens["access_token"], tokens["refresh_token"])
    if code != 200:
        print("TEST[get many users with correct data] failed")
    else:
        users, code = get_many_users(
            tokens["access_token"], "", "", random_email, None, None, None)
        if code != 200:
            print("TEST[get many users with correct data] failed")
        else:
            print("TEST[get many users with correct data] succeeded")

    # get one user
    tokens, code = refresh(tokens["access_token"], tokens["refresh_token"])
    if code != 200:
        print("TEST[get one user with correct data] failed")
    else:
        users, code = get_one_user(
            tokens["access_token"], users["data"]["data"][0]["_id"]["$oid"])
        if code != 200:
            print("TEST[get one user with correct data] failed")
        else:
            print("TEST[get one user with correct data] succeeded")

    # update user
    tokens, code = refresh(tokens["access_token"], tokens["refresh_token"])
    if code != 200:
        print("TEST[update user with correct data] failed")
    else:
        _, code = update_admin(tokens["access_token"], users["data"]
                               [0]["_id"]["$oid"], "test", "test", None, "testtest", None)
        if code != 200:
            print("TEST[update user with correct data] failed")
        else:
            print("TEST[update user with correct data] succeeded")

    # delete user
    tokens, code = refresh(tokens["access_token"], tokens["refresh_token"])
    if code != 200:
        print("TEST[delete user with correct data] failed")
    else:
        _, code = delete_admin(
            tokens["access_token"], users["data"][0]["_id"]["$oid"])
        if code != 200:
            print("TEST[delete user with correct data] failed")
        else:
            print("TEST[delete user with correct data] succeeded")

    # update me
    tokens, code = refresh(tokens["access_token"], tokens["refresh_token"])
    if code != 200:
        print("TEST[update my profile with correct data] failed")
    else:
        _, code = update_me(
            tokens["access_token"], "admiiiiiiiiin", "admiiiiiiiiiiiiiiin", None, None)
        if code != 200:
            print("TEST[update my profile with correct data] failed")
        else:
            print("TEST[update my profile with correct data] succeeded")

    # get me
    tokens, code = refresh(tokens["access_token"], tokens["refresh_token"])
    if code != 200:
        print("TEST[get my profile with correct data] failed")
    else:
        _, code = get_my_acc(tokens["access_token"])
        if code != 200:
            print("TEST[get my profile with correct data] failed")
        else:
            print("TEST[get my profile with correct data] succeeded")

    # signout from admin
    tokens, code = refresh(tokens["access_token"], tokens["refresh_token"])
    if code != 200:
        print("TEST[signout from admin with correct data] failed")
    else:
        _, code = signout(tokens["access_token"], tokens["refresh_token"])
        if code != 200:
            print("TEST[signout from admin with correct data] failed")
        else:
            print("TEST[signout from admin with correct data] succeeded")
    # wait for keyboard input then return
    input("Press Enter to continue...")
    return


def clear_screen():
    if 'linux' in sys.platform:
        os.system('clear')
        return
    elif 'win' in sys.platform:
        os.system('cls')
        return
    sys.exit('YOUR OS IS NOT SUPPORTED')

menu_tokens = None
menu_token_has_been_used = False

def start_menu():
    menu_items = {
        'type': 'list',
        'name': 'start_menu',
        'message': 'How do you want to test the api?',
        'choices': ['manually', 'automatically', Separator(), 'exit'],
    }
    return prompt(menu_items)['start_menu']


def manual_menu_pipeline():
    clear_screen()
    insert_menu = {
        'type': 'list',
        'name': 'insert_menu',
        'message': 'What are you willing too do?',
        'choices': ['POST /api/users', 'DELETE /api/users', 'DELETE /api/users/\{id\}',
                    'GET /api/users', 'GET /api/users/my_acc', 'GET /api/users/\{id\}',
                    'PUT /api/users/\{id\}',
                    'PUT /api/users', 'POST /api/auth/sign_in',
                    'POST /api/auth/sign_up', 'POST /api/auth/sign_out',
                    'POST /api/auth/refresh'
                    ],
    }
    choice = prompt(insert_menu)['insert_menu']
    clear_screen()
    if choice == 'POST /api/users':
        handle_create_user()
    elif choice == 'DELETE /api/users':
        pass
    elif choice == 'DELETE /api/users/\{id\}':
        handle_delete_admin()
    elif choice == 'GET /api/users':
        handle_get_many_users()
    elif choice == 'GET /api/users/my_acc':
        pass
    elif choice == 'GET /api/users/\{id\}':
        handle_get_one_user()
    elif choice == 'PUT /api/users/\{id\}':
        handle_update_admin()
    elif choice == 'PUT /api/users':
        pass
    elif choice == 'POST /api/auth/sign_in':
        handle_signin()
    elif choice == 'POST /api/auth/sign_up':
        handle_signup()
    elif choice == 'POST /api/auth/sign_out':
        handle_signout()
    elif choice == 'POST /api/auth/refresh':
        handle_refresh()


def handle_signup():
    global menu_token_has_been_used, debug_global, menu_tokens
    questions = [
        {
            'type': 'input',
            'name': 'first_name',
            'message': 'What\'s the user\'s name?',
        },
        {
            'type': 'input',
            'name': 'last_name',
            'message': 'What\'s the user\'s last name?',
        },
        {
            'type': 'input',
            'name': 'email',
            'message': 'What\'s the user\'s email?',
        },
        {
            'type': 'input',
            'name': 'password',
            'message': 'What\'s the user\'s password?',
        },
    ]
    answers = prompt(questions)
    if answers['first_name'] == "":
        print('PLEASE ANSWER ALL :)')
        return 

    if answers['last_name'] == "": 
        print('PLEASE ANSWER ALL :)')
        return 

    if answers['email'] == "": 
        print('PLEASE ANSWER ALL :)')
        return 

    if answers['password'] == "": 
        print('PLEASE ANSWER ALL :)')
        return 

    debug_global = True
    _, code = signup(answers['first_name'], answers['last_name'], answers['email'], answers['password'])
    print(f"response code: {code}")
    debug_global = False

def handle_signin():
    global menu_token_has_been_used, debug_global, menu_tokens
    questions = [
        {
            'type': 'input',
            'name': 'email',
            'message': 'What\'s the user\'s email?',
        },
        {
            'type': 'input',
            'name': 'password',
            'message': 'What\'s the user\'s password?',
        },
    ]
    answers = prompt(questions)
    if answers['email'] == "": 
        print('PLEASE ANSWER ALL :)')
        return 

    if answers['password'] == "": 
        print('PLEASE ANSWER ALL :)')
        return 

    debug_global = True
    _, code = signin(answers['email'], answers['password'])
    print(f"response code: {code}")
    debug_global = False

def handle_signout():
    global menu_token_has_been_used, debug_global, menu_tokens
    questions = [
        {
            'type': 'input',
            'name': 'access_token',
            'message': 'What\'s the user\'s access token?',
        },
        {
            'type': 'input',
            'name': 'refresh_token',
            'message': 'What\'s the user\'s refresh token?',
        },
    ]
    answers = prompt(questions)
    if answers['access_token'] == "": 
        print('PLEASE ANSWER ALL :)')
        return 

    if answers['refresh_token'] == "": 
        print('PLEASE ANSWER ALL :)')
        return 

    debug_global = True
    _, code = signout(answers['access_token'], answers['refresh_token'])
    print(f"response code: {code}")
    debug_global = False

def handle_refresh():
    global menu_token_has_been_used, debug_global, menu_tokens
    questions = [
        {
            'type': 'input',
            'name': 'access_token',
            'message': 'What\'s the user\'s access token?',
        },
        {
            'type': 'input',
            'name': 'refresh_token',
            'message': 'What\'s the user\'s refresh token?',
        },
    ]
    answers = prompt(questions)
    if answers['access_token'] == "": 
        print('PLEASE ANSWER ALL :)')
        return 

    if answers['refresh_token'] == "": 
        print('PLEASE ANSWER ALL :)')
        return 

    debug_global = True
    tokens, code = refresh(answers['access_token'], answers['refresh_token'])
    print(f"response code: {code}")
    debug_global = False

def handle_get_email_and_password():
    global menu_token_has_been_used, debug_global, menu_tokens
    questions = [
        {
            'type': 'input',
            'name': 'email',
            'message': 'What\'s the user\'s email?',
        },
        {
            'type': 'input',
            'name': 'password',
            'message': 'What\'s the user\'s password?',
        },
    ]
    answers = prompt(questions)
    if answers['email'] == "": 
        print('PLEASE ANSWER ALL :)')
        return 

    if answers['password'] == "": 
        print('PLEASE ANSWER ALL :)')
        return 

    debug_global = True
    menu_tokens, code = signin(answers['email'], answers['password'])
    menu_token_has_been_used = True
    print(f"response code: {code}")
    debug_global = False

def handle_create_user():
    global menu_token_has_been_used, debug_global, menu_tokens
    questions = [
        {
            'type': 'input',
            'name': 'first_name',
            'message': 'What\'s the user\'s name?',
        },
        {
            'type': 'input',
            'name': 'last_name',
            'message': 'What\'s the user\'s last name?',
        },
        {
            'type': 'input',
            'name': 'email',
            'message': 'What\'s the user\'s email?',
        },
        {
            'type': 'input',
            'name': 'password',
            'message': 'What\'s the user\'s password?',
        },
        {
            'type': 'input',
            'name': 'access_level',
            'message': 'What\'s the user\'s access level?',
        }
    ]
    answers = prompt(questions)
    if answers['first_name'] == "":
        print('PLEASE ANSWER ALL :)')
        return 

    if answers['last_name'] == "": 
        print('PLEASE ANSWER ALL :)')
        return 

    if answers['email'] == "": 
        print('PLEASE ANSWER ALL :)')
        return 

    if answers['password'] == "": 
        print('PLEASE ANSWER ALL :)')
        return 

    if answers['access_level'] == "":
        print('PLEASE ANSWER ALL :)')
        return
    
    if menu_token_has_been_used:
        menu_tokens, _ = refresh(menu_tokens['access_token'], menu_tokens['refresh_token'])

    menu_token_has_been_used = True
    debug_global = True
    _, code = create_user(menu_tokens['access_token'], answers['first_name'], answers['last_name'], answers['email'], answers['password'], int(answers['access_level']))
    print(f"response code: {code}")
    debug_global = False

def handle_update_admin():
    global menu_token_has_been_used, debug_global, menu_tokens
    questions = [
        {
            'type': 'input',
            'name': 'first_name',
            'message': 'What\'s the user\'s name?',
        },
        {
            'type': 'input',
            'name': 'last_name',
            'message': 'What\'s the user\'s last name?',
        },
        {
            'type': 'input',
            'name': 'email',
            'message': 'What\'s the user\'s email?',
        },
        {
            'type': 'input',
            'name': 'password',
            'message': 'What\'s the user\'s password?',
        },
        {
            'type': 'input',
            'name': 'access_level',
            'message': 'What\'s the user\'s access level?',
        }
    ]
    answers = prompt(questions)
    #check if at least one of them is answered
    if not (answers['first_name'] or answers['last_name'] or answers['email'] or answers['password'] or answers['access_level']):
        print('PLEASE ANSWER AT LEAST ONE :)')
        return
    
    if menu_token_has_been_used:
        menu_tokens, _ = refresh(menu_tokens['access_token'], menu_tokens['refresh_token'])
    menu_token_has_been_used = True

    debug_global = True
    _, code = update_admin(menu_tokens['access_token'], answers['first_name'], answers['last_name'], answers['email'], answers['password'], int(answers['access_level']))
    print(f"response code: {code}")
    debug_global = False

def handle_delete_admin():
    global menu_token_has_been_used, debug_global, menu_tokens
    questions = [
        {
            'type': 'input',
            'name': 'id',
            'message': 'What\'s the user\'s id?',
        }
    ]
    answers = prompt(questions)
    if answers['id'] == "":
        print('PLEASE ANSWER ALL :)')
        return
    
    if menu_token_has_been_used:
        menu_tokens, _ = refresh(menu_tokens['access_token'], menu_tokens['refresh_token'])
    menu_token_has_been_used = True

    debug_global = True
    _, code = delete_admin(menu_tokens['access_token'], answers['id'])
    print(f"response code: {code}")
    debug_global = False

def handle_get_many_users():
    global menu_token_has_been_used, debug_global, menu_tokens
    #ask for filters
    questions = [
        {
            'type': 'input',
            'name': 'first_name',
            'message': 'What\'s the user\'s name?',
        },
        {
            'type': 'input',
            'name': 'last_name',
            'message': 'What\'s the user\'s last name?',
        },
        {
            'type': 'input',
            'name': 'email',
            'message': 'What\'s the user\'s email?',
        },
        {
            'type': 'input',
            'name': 'access_level',
            'message': 'What\'s the user\'s access level?',
        },
        {
            'type': 'input',
            'name': 'access_level_cmp',
            'message': 'How to compare the access level?',
        }
    ]
    answers = prompt(questions)
    
    if menu_token_has_been_used:
        menu_tokens, _ = refresh(menu_tokens['access_token'], menu_tokens['refresh_token'])
    menu_token_has_been_used = True

    debug_global = True
    users, code = get_many_users(menu_tokens['access_token'], answers['first_name'], answers['last_name'], answers['email'], None, answers['access_level_cmp'])
    print(f"response code: {code}")
    debug_global = False

def handle_get_one_user():
    global menu_token_has_been_used, debug_global, menu_tokens
    questions = [
        {
            'type': 'input',
            'name': 'id',
            'message': 'What\'s the user\'s id?',
        }
    ]
    answers = prompt(questions)
    if answers['id'] == "":
        print('PLEASE ANSWER ALL :)')
        return
    
    if menu_token_has_been_used:
        menu_tokens, _ = refresh(menu_tokens['access_token'], menu_tokens['refresh_token'])
    menu_token_has_been_used = True

    debug_global = True
    user, code = get_one_user(menu_tokens['access_token'], answers['id'])
    print(f"response code: {code}")
    debug_global = False


while(True):
    clear_screen()
    func = start_menu()
    if func == 'manually': 
        handle_get_email_and_password()
        manual_menu_pipeline()
    if func == 'automatically':
        automated_test_pipeline()
        pass

    if func == 'exit':
        sys.exit(0)

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
# tokens = signin("test@test.com", "testtest")

# get_my_acc(tokens['access_token'])
# tokens = refresh(tokens['access_token'], tokens['refresh_token'])

# for i in range(10):
#     create_user(tokens['access_token'], "test" + str(i), "test" + str(i), "test" + str(i) + "@test.com", "testtest", 1)
#     tokens = refresh(tokens['access_token'], tokens['refresh_token'])

# users = get_many_users(tokens['access_token'], "", "", "", 1, 0, 1, 10)
# print(json.dumps(users, indent=4))
# tokens = refresh(tokens['access_token'], tokens['refresh_token'])
# #update created users
# for i in range(10):
#     update_admin(tokens['access_token'], users["data"]["data"][i]["_id"]["$oid"], "test" + str(i * 2), "test" + str(i * 2), "test" + str(i + 100) + "@test.com", "testtest", access_level=1)
#     tokens = refresh(tokens['access_token'], tokens['refresh_token'])

# #delete created users
# users = get_many_users(tokens['access_token'], "", "", "", 1, 0, 1, 10)
# tokens = refresh(tokens['access_token'], tokens['refresh_token'])
# #test getting one user
# get_one_user(tokens['access_token'], users["data"]["data"][0]["_id"]["$oid"])
# tokens = refresh(tokens['access_token'], tokens['refresh_token'])
# #test deleting one user
# delete_admin(tokens['access_token'], users["data"]["data"][0]["_id"]["$oid"])
# tokens = refresh(tokens['access_token'], tokens['refresh_token'])
# #test getting all users
# users = get_many_users(tokens['access_token'], "", "", "", 1, 0, 1, 10)
# print(json.dumps(users, indent=4))
# tokens = refresh(tokens['access_token'], tokens['refresh_token'])
# for user in users["data"]["data"]:
#     delete_admin(tokens['access_token'], user["_id"]["$oid"])
#     tokens = refresh(tokens['access_token'], tokens['refresh_token'])


# #create a user and sign in with it then delete the user and test login tokens they should fail
# print("\n\n\n")
# print("create a user and sign in with it then delete the user and test login tokens they should fail")
# print("creating user")
# create_user(tokens['access_token'], "test", "test", "testaccdelete@delete.com", "testtest", 1)
# print("user created")
# print("signing in")
# user_tokens = signin("testaccdelete@delete.com", "testtest")
# print("user signed in")
# tokens = refresh(tokens['access_token'], tokens['refresh_token'])
# print("retrieving user")
# user_acc = get_many_users(tokens['access_token'], "", "", "", 1, 0, 1, 10)
# print("user retrieved")
# tokens = refresh(tokens['access_token'], tokens['refresh_token'])
# print("deleting user")
# delete_admin(tokens['access_token'], user_acc["data"]["data"][0]["_id"]["$oid"])
# print("user deleted")
# tokens = refresh(tokens['access_token'], tokens['refresh_token'])
# delete_admin(tokens['access_token'], user_acc["data"]["data"][0]["_id"]["$oid"])
# tokens = refresh(tokens['access_token'], tokens['refresh_token'])
# get_one_user(tokens['access_token'], user_acc["data"]["data"][0]["_id"]["$oid"])
# tokens = refresh(tokens['access_token'], tokens['refresh_token'])
# print("test delete")
# update_me(user_tokens['access_token'], "test", "test", "", "")


# signout(tokens['access_token'], tokens['refresh_token'])


# # signup("test", "test", "test@test.com", "testtest")
# # signup("test", "test", "test_badmail.com", "testtest")
# # signup("test", "test", "testtesttest@test.com", "shortp")
# # signup("test", "test", "testttseqwettt@test.com", "testtest")


# # signin("nono@nono.com", "nonononononono")
# # tokens = signin("test@test.com", "testtest")

# # get_many_users(tokens['access_token'], "", "", "", 1, 0, 1, 10)

# # tokens = refresh(tokens['access_token'], tokens['refresh_token'])

# # signout(tokens['access_token'], tokens['refresh_token'])
# # signout(tokens['access_token'], tokens['refresh_token'])
