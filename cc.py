import json
import requests

room = "Room 1" # Room Name
frab = "https://timetable.flawcon.xyz/2019/schedule/export/schedule.json" # Timetable/Schedule URL

sched = requests.get(frab).json()["schedule"]["conference"]["days"][0]["rooms"]

z = ""

for i in sched[room]:
    if i["do_not_record"] == False:
        print("From-until\t" + i["title"])

        if len(i["persons"]) == 1:
            for y in i["persons"]:
                z = z + y["name"]
        elif len(i["persons"]) > 1:
            for y in i["persons"][:-1]:
               z = z + y["name"] + ", "
            z = z + i["persons"][len(i["persons"])-1]["name"]
        else:
            print("Error")

        print("From-until\t- " + z)
        z = ""