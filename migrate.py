import psycopg2
import sqlite3

old_db = ""  # file path to old db
post_conn = psycopg2.connect("dbname='' user='' host=''  password=''")
lite_con = sqlite3.connect(old_db)
lite_cur = lite_con.cursor()
lite_cur.execute("SELECT * FROM USERS;")
users = lite_cur.fetchall()
post_cur = post_conn.cursor()
for i in users:
    post_cur.execute("INSERT INTO users (discord_id, first_name, last_name, email, school_code) VALUES (%s, %s, %s, "
                     "%s, %s);", (i[0], i[1], i[2], i[3], i[4]))
post_conn.commit()
post_cur.close()
post_conn.close()
lite_con.commit()
lite_cur.close()
lite_con.close()
