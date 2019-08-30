import pickle 
import json


def main():
    val = json.loads("""{
            "userid": 103609,
            "verified": true,
            "friendly_name": "Jason",
            "access_privileges": [
              "user",
              "admin"
            ]
        }""") # load the json string as dictionary

    # open "user.pkl" to write binary data (= wb)
    with open("user.pkl", "wb") as out:
        pickle.dump(val, out) # write the dictionary

if __name__ == '__main__':
    main()