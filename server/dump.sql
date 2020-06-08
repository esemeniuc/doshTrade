PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE __diesel_schema_migrations (version VARCHAR(50) PRIMARY KEY NOT NULL,run_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP);
INSERT INTO __diesel_schema_migrations VALUES('20200423002047','2020-05-04 15:24:04');
INSERT INTO __diesel_schema_migrations VALUES('20200426185111','2020-05-04 15:24:04');
INSERT INTO __diesel_schema_migrations VALUES('20200427041042','2020-05-04 15:24:04');
CREATE TABLE users
(
    id                INTEGER   NOT NULL PRIMARY KEY,
    first_name        VARCHAR   NOT NULL,
    last_name         VARCHAR   NOT NULL,
    email             VARCHAR   NOT NULL UNIQUE,
    password          VARCHAR   NOT NULL,
    auth_bearer_token VARCHAR   NOT NULL,
    created_at        TIMESTAMP NOT NULL
);
INSERT INTO users VALUES(1,'bob','lob','a@a.com','pass','eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOjEsImV4cCI6MTU5MDAyNDY3OSwibm9uY2UiOiI5YVJLNFlFIn0.ngSU0Ddl0W802NQPlin8KDNKLmudbxNtpBOekHLFOc9wxKWjKp48VQRnP6I9Z5vIx_tSuxiz7gIZYgh-ZDc6-9nDRAc7PzpmGvkUXxe-2hCdgRNo8Z7c1eE6oclWAQqM7UN5BM5X_SnkVoRLOqTqLs_QvlrTd43kmP-eOvW-ZE240MYuTr1_fSpKEkHHv2JxeNRZqRAhj7ztpRnadRc1eGdGRjnkSThTmXLMt_rRdWyeKr9hdjpwFZhTZaRLTXuSRp-W-T_IdfsQRVlkvM9fujo7pbtfXdiF8hVDZJndu0zzPUJBiWlROCsMqrQ403aHHJqv6iQ0LCVLFjncfa0exA','2020-05-04 15:24:04.615002127');
CREATE TABLE properties
(
    id           VARCHAR   NOT NULL PRIMARY KEY,
    website_name VARCHAR   NOT NULL,
    website_url  VARCHAR   NOT NULL,
    user_id      INTERGER  NOT NULL,
    created_at   TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);
INSERT INTO properties VALUES('1-1','foo','foo.com',1,'2020-05-06 15:32:18.767391349');
INSERT INTO properties VALUES('1-2','foo','foo.com',1,'2020-05-07 01:04:02.253895122');
CREATE TABLE events
(
    id          INTEGER   NOT NULL PRIMARY KEY,
    url         VARCHAR   NOT NULL,
    ip          VARCHAR   NOT NULL,
    user_agent  VARCHAR   NOT NULL,
    fingerprint VARCHAR   NOT NULL,
    is_private  BOOLEAN   NOT NULL,
    property_id VARCHAR   NOT NULL,
    created_at  TIMESTAMP NOT NULL,
    FOREIGN KEY (property_id) REFERENCES properties (id)
);
INSERT INTO events VALUES(1,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=1skuqjfprth4b9h0m9qkn8l79k','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','b530673b9e7c0ac953aa3c833458b8ea',0,'1-1','2020-05-06 21:21:46.423553328');
INSERT INTO events VALUES(2,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=1skuqjfprth4b9h0m9qkn8l79k','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','b530673b9e7c0ac953aa3c833458b8ea',0,'1-1','2020-05-06 21:22:10.894591306');
INSERT INTO events VALUES(3,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=1skuqjfprth4b9h0m9qkn8l79k','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','b530673b9e7c0ac953aa3c833458b8ea',0,'1-1','2020-05-06 21:22:33.333109886');
INSERT INTO events VALUES(4,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=1skuqjfprth4b9h0m9qkn8l79k','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','b530673b9e7c0ac953aa3c833458b8ea',0,'1-1','2020-05-06 21:22:52.921667780');
INSERT INTO events VALUES(5,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=aq0900q0mg5n0bgr1bu9p9ehmj','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','910e3e5e086a249d441ef7ae4f336476',1,'1-1','2020-05-06 21:23:25.738610685');
INSERT INTO events VALUES(6,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=aq0900q0mg5n0bgr1bu9p9ehmj','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','910e3e5e086a249d441ef7ae4f336476',1,'1-1','2020-05-06 21:23:27.452553888');
INSERT INTO events VALUES(7,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=aq0900q0mg5n0bgr1bu9p9ehmj','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','910e3e5e086a249d441ef7ae4f336476',1,'1-1','2020-05-06 21:23:28.096631138');
INSERT INTO events VALUES(8,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=aq0900q0mg5n0bgr1bu9p9ehmj','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','910e3e5e086a249d441ef7ae4f336476',1,'1-1','2020-05-06 21:23:28.551323930');
INSERT INTO events VALUES(9,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=aq0900q0mg5n0bgr1bu9p9ehmj','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','910e3e5e086a249d441ef7ae4f336476',1,'1-1','2020-05-06 21:23:28.733419656');
INSERT INTO events VALUES(10,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=aq0900q0mg5n0bgr1bu9p9ehmj','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','910e3e5e086a249d441ef7ae4f336476',1,'1-1','2020-05-06 21:23:28.944196734');
INSERT INTO events VALUES(11,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=aq0900q0mg5n0bgr1bu9p9ehmj','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','910e3e5e086a249d441ef7ae4f336476',1,'1-1','2020-05-06 21:23:29.251104861');
INSERT INTO events VALUES(12,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=aq0900q0mg5n0bgr1bu9p9ehmj','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','910e3e5e086a249d441ef7ae4f336476',1,'1-1','2020-05-06 21:23:30.104954813');
INSERT INTO events VALUES(13,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=965imufhlp278aakvs6roa9jos','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','b530673b9e7c0ac953aa3c833458b8ea',0,'1-1','2020-05-06 22:25:41.753506969');
INSERT INTO events VALUES(14,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=965imufhlp278aakvs6roa9jos','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','b530673b9e7c0ac953aa3c833458b8ea',0,'1-1','2020-05-06 22:25:53.850636700');
INSERT INTO events VALUES(15,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=965imufhlp278aakvs6roa9jos','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','b530673b9e7c0ac953aa3c833458b8ea',0,'1-1','2020-05-06 22:25:54.747861727');
INSERT INTO events VALUES(16,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=965imufhlp278aakvs6roa9jos','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','b530673b9e7c0ac953aa3c833458b8ea',0,'1-1','2020-05-06 22:25:55.390352448');
INSERT INTO events VALUES(17,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=965imufhlp278aakvs6roa9jos','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','910e3e5e086a249d441ef7ae4f336476',1,'1-1','2020-05-06 22:26:04.968852589');
INSERT INTO events VALUES(18,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=965imufhlp278aakvs6roa9jos','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','910e3e5e086a249d441ef7ae4f336476',1,'1-1','2020-05-06 22:26:09.258086032');
INSERT INTO events VALUES(19,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=965imufhlp278aakvs6roa9jos','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','910e3e5e086a249d441ef7ae4f336476',1,'1-1','2020-05-06 22:26:10.039926379');
INSERT INTO events VALUES(20,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=sftnt4sfuutujmbeqcuudcjdqp','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','b530673b9e7c0ac953aa3c833458b8ea',0,'1-1','2020-05-07 16:41:21.121881757');
INSERT INTO events VALUES(21,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=1skuqjfprth4b9h0m9qkn8l79k|localhost:8000|Mozilla/5.0','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','b530673b9e7c0ac953aa3c833458b8ea',0,'1-1','2020-05-07 16:41:25.142577923');
INSERT INTO events VALUES(22,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=5bd12oqgrno7dqkl3dmb88dlrg','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','b530673b9e7c0ac953aa3c833458b8ea',0,'1-1','2020-05-07 16:42:43.793246101');
INSERT INTO events VALUES(23,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=5bd12oqgrno7dqkl3dmb88dlrg','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','b530673b9e7c0ac953aa3c833458b8ea',0,'1-1','2020-05-07 16:42:44.937985385');
INSERT INTO events VALUES(24,'http://localhost:63342/lib-js-dashboard/example.html?_ijt=5bd12oqgrno7dqkl3dmb88dlrg','localhost:8000','Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0','910e3e5e086a249d441ef7ae4f336476',1,'1-1','2020-05-07 16:42:49.705451077');
COMMIT;
