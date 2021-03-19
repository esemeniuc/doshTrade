pub type DbPool = sqlx::PgPool;

pub async fn seed(conn: &DbPool) -> sqlx::Result<sqlx::postgres::PgDone> {
    // Read migrations from a local folder: ./migrations
    // let m = sqlx::migrate::Migrator::new(std::path::Path::new("./migrations")).await?;
    // m.run(conn).await?;
    sqlx::migrate!("./migrations").run(conn).await?;

    let stocks_list = [
        ("AAPL", "Apple"),
        ("FB", "Facebook"),
        ("GLD", "Gold"),
        ("GOOG", "Google"),
        ("LIT", "Lithium"),
        ("NFLX", "Netflix"),
        ("SLV", "Silver"),
        ("SQ", "Square"),
        ("TSLA", "Tesla"),
        ("TSM", "Taiwan Semiconductor"),
        ("UVXY", "Ultra Volatility Index"),
        ("YI", "111 Inc"),
        ("ZM", "Zoom"),
    ];

    let inserts = stocks_list.iter().map(|stock| {
        sqlx::query("INSERT INTO stocks VALUES (DEFAULT, $1, $2) ON CONFLICT DO NOTHING")
            .bind(stock.0)
            .bind(stock.1)
            .execute(conn)
    });

    return futures::future::join_all(inserts).await.into_iter().fold(
        sqlx::Result::Ok(sqlx::postgres::PgDone::default()),
        |acc, curr| {
            if Result::is_ok(&curr) {
                acc
            } else {
                curr
            }
        },
    );


    // sqlx::query("insert into clients VALUES(DEFAULT, 'endpoint', 'p256', 'auth', CURRENT_TIMESTAMP)").execute(conn).await;
    // sqlx::query("insert into client_subscriptions VALUES(DEFAULT, 1,1,CURRENT_TIMESTAMP - interval '1 month')").execute(conn).await;
    // sqlx::query("UPDATE client_subscriptions SET last_sent = CURRENT_TIMESTAMP WHERE id IN (SELECT unnest($1))")
    //     .bind(vec![1, 2, 3])
    //     .execute(conn).await;

    return Ok(sqlx::postgres::PgDone::default());
    /*

    c1 = {}
    c1[15]='6/8/2020;82.56 83.40 81.83 83.37|95,407,900|72.06+10+17';c1[19]='6/9/2020;83.04 86.40 83.00 86.00|147,712,364|77.10+17+21';c1[24]='6/10/2020;86.98 88.69 86.52 88.21|165,423,144|80.32+22+26';c1[29]='6/11/2020;87.33 87.77 83.87 83.98|198,270,700|62.29+27+31';c1[33]='6/12/2020;86.18 86.95 83.56 84.70|200,146,052|63.79+31+35';c1[38]='6/15/2020;83.31 86.42 83.15 85.75|138,808,920|65.90+36+40';c1[43]='6/16/2020;87.87 88.30 86.18 88.02|164,436,052|69.98+41+45';c1[47]='6/17/2020;88.79 88.85 87.77 87.90|114,406,504|69.50+45+49';c1[52]='6/18/2020;87.85 88.36 87.31 87.93|94,421,232|69.56+50+54';c1[57]='6/19/2020;88.66 89.14 86.29 87.43|264,475,808|67.36+55+59';c1[61]='6/22/2020;87.84 89.87 87.79 89.72|134,038,636|71.75+59+63';c1[66]='6/23/2020;91.00 93.10 90.57 91.63|211,257,012|74.81+64+68';c1[71]='6/24/2020;91.25 92.20 89.63 90.02|190,982,340|68.10+69+73';c1[75]='6/25/2020;90.18 91.25 89.39 91.21|136,222,032|70.23+73+77';c1[80]='6/26/2020;91.10 91.33 88.26 88.41|205,256,844|60.12+78+82';c1[85]='6/29/2020;88.31 90.54 87.82 90.45|129,839,052|64.16+83+87';c1[89]='6/30/2020;90.02 91.50 90.00 91.20|139,190,968|65.55+87+91';c1[94]='7/1/2020;91.28 91.84 90.98 91.03|109,052,836|64.93+92+96';c1[99]='7/2/2020;91.96 92.62 90.91 91.03|111,057,880|64.93+97+101';c1[103]='7/6/2020;92.50 93.95 92.47 93.46|115,944,308|69.64+101+105';c1[108]='7/7/2020;93.85 94.66 93.06 93.17|112,827,996|68.46+106+110';c1[112]='7/8/2020;94.18 95.38 94.09 95.34|115,523,504|72.24+110+114';c1[117]='7/9/2020;96.26 98.48 94.67 95.68|123,026,536|72.79+115+119';c1[122]='7/10/2020;95.34 95.98 94.71 95.92|90,257,320|73.19+120+124';c1[126]='7/13/2020;97.27 99.96 95.26 95.48|188,132,988|71.10+124+128';c1[131]='7/14/2020;94.84 97.26 93.88 97.06|166,204,064|73.96+129+133';c1[136]='7/15/2020;98.99 99.25 96.49 97.73|150,660,352|75.09+134+138';c1[140]='7/16/2020;96.56 97.41 95.91 96.52|107,259,048|69.28+138+142';c1[145]='7/17/2020;96.99 97.15 95.84 96.33|92,187,020|68.36+143+147';c1[150]='7/20/2020;96.42 98.50 96.06 98.36|89,588,832|72.47+148+152';c1[154]='7/21/2020;99.17 99.25 96.74 97.00|102,509,388|66.27+152+156';c1[159]='7/22/2020;96.69 97.98 96.60 97.27|82,360,080|66.88+157+161';c1[164]='7/23/2020;97.00 97.08 92.01 92.85|197,004,432|50.75+162+166';c1[168]='7/24/2020;90.99 92.97 89.15 92.62|185,438,864|50.07+166+170';c1[173]='7/27/2020;93.71 94.91 93.48 94.81|117,879,876|56.08+171+175';c1[178]='7/28/2020;94.37 94.55 93.25 93.25|101,245,172|51.36+176+180';c1[182]='7/29/2020;93.75 95.23 93.71 95.04|87,272,372|55.94+180+184';c1[187]='7/30/2020;94.19 96.30 93.77 96.19|126,407,408|58.65+185+189';c1[192]='7/31/2020;102.88 106.42 100.83 106.26|374,295,468|73.80+190+194';c1[196]='8/3/2020;108.20 111.64 107.89 108.94|307,269,208|76.29+194+198';c1[201]='8/4/2020;109.13 110.79 108.39 109.67|172,792,368|76.93+199+203';c1[206]='8/5/2020;109.38 110.39 108.90 110.06|120,102,788|77.29+204+208';c1[210]='8/6/2020;110.41 114.41 109.80 113.90|202,428,900|80.47+208+212';c1[215]='8/7/2020;113.21 113.68 110.29 111.11|198,045,612|72.52+213+217';c1[220]='8/10/2020;112.60 113.78 110.00 112.73|212,403,424|74.12+218+222';c1[224]='8/11/2020;111.97 112.48 109.11 109.38|185,915,844|65.61+222+226';c1[229]='8/12/2020;110.50 113.28 110.30 113.01|164,514,552|69.67+227+231';c1[234]='8/13/2020;114.43 116.04 113.93 115.01|201,950,304|71.66+232+236';c1[238]='8/14/2020;114.83 115.00 113.05 114.91|165,565,208|71.40+236+240';c1[243]='8/17/2020;116.06 116.09 113.96 114.61|117,725,656|70.60+241+245';c1[248]='8/18/2020;114.35 116.00 114.01 115.56|104,095,732|71.69+246+250';c1[252]='8/19/2020;115.98 117.16 115.61 115.71|144,547,048|71.86+250+254';c1[257]='8/20/2020;115.75 118.39 115.73 118.28|125,313,344|74.76+255+259';c1[262]='8/21/2020;119.26 124.87 119.25 124.37|338,054,640|80.02+260+264';c1[266]='8/24/2020;128.70 128.79 123.94 125.86|343,339,388|81.05+264+268';c1[271]='8/25/2020;124.70 125.18 123.05 124.83|208,941,716|78.02+269+273';c1[276]='8/26/2020;126.18 126.99 125.08 126.52|161,004,836|79.39+274+278';c1[280]='8/27/2020;127.14 127.49 123.83 125.01|154,146,696|74.93+278+282';c1[285]='8/28/2020;126.01 126.44 124.58 124.81|187,629,916|74.33+283+287';c1[290]='8/31/2020;127.58 131.00 126.00 129.04|223,505,733|78.26+288+292';c1[294]='9/1/2020;132.76 134.80 130.53 134.18|150,698,946|81.89+292+296';c1[299]='9/2/2020;137.59 137.98 127.00 131.40|198,233,370|74.63+297+301';c1[304]='9/3/2020;126.91 128.84 120.50 120.88|250,623,616|54.83+302+306';c1[308]='9/4/2020;120.07 123.70 110.89 120.96|332,607,163|54.93+306+310';c1[313]='9/8/2020;113.95 118.99 112.68 112.82|226,149,287|44.39+311+315';c1[318]='9/9/2020;117.26 119.14 115.26 117.32|172,144,189|50.09+316+320';c1[322]='9/10/2020;120.36 120.50 112.50 113.49|182,274,391|45.79+320+324';c1[327]='9/11/2020;114.57 115.23 110.00 112.00|180,860,325|44.20+325+329';c1[332]='9/14/2020;114.72 115.93 112.80 115.36|138,248,838|48.53+330+334';c1[336]='9/15/2020;118.33 118.83 113.61 115.54|182,985,051|48.77+334+338';c1[341]='9/16/2020;115.23 116.00 112.04 112.13|153,804,096|44.69+339+343';c1[345]='9/17/2020;109.72 112.20 108.71 110.34|176,909,480|42.68+343+347';c1[350]='9/18/2020;110.40 110.88 106.09 106.84|284,477,372|38.98+348+352';c1[355]='9/21/2020;104.54 110.19 103.10 110.08|193,321,297|43.83+353+357';c1[359]='9/22/2020;112.68 112.86 109.16 111.81|181,592,150|46.29+357+361';c1[364]='9/23/2020;111.62 112.11 106.77 107.12|148,510,882|41.05+362+366';c1[369]='9/24/2020;105.17 110.25 105.00 108.22|167,743,349|42.69+367+371';c1[373]='9/25/2020;108.43 112.44 107.67 112.28|149,981,441|48.39+371+375';c1[378]='9/28/2020;115.01 115.32 112.78 114.96|136,441,425|51.80+376+380';c1[383]='9/29/2020;114.55 115.31 113.57 114.09|98,636,389|50.63+381+385';c1[387]='9/30/2020;113.79 117.26 113.62 115.81|141,667,323|52.90+385+389';c1[392]='10/1/2020;117.64 117.72 115.83 116.79|114,958,231|54.19+390+394';c1[397]='10/2/2020;112.89 115.37 112.22 113.02|144,711,986|48.67+395+399';c1[401]='10/5/2020;113.91 116.65 113.55 116.50|105,392,594|53.39+399+403';c1[406]='10/6/2020;115.70 116.12 112.25 113.16|159,519,440|48.75+404+408';c1[411]='10/7/2020;114.62 115.55 114.13 115.08|96,135,446|51.37+409+413';c1[415]='10/8/2020;116.25 116.40 114.59 114.97|82,322,071|51.21+413+417';c1[420]='10/9/2020;115.28 117.00 114.92 116.97|100,506,865|54.03+418+422';c1[425]='10/12/2020;120.06 125.18 119.28 124.40|236,262,242|62.67+423+427';c1[429]='10/13/2020;125.27 125.39 119.65 121.10|260,497,701|57.50+427+431';c1[434]='10/14/2020;121.00 123.03 119.62 121.19|150,268,765|57.60+432+436';c1[439]='10/15/2020;118.72 121.20 118.15 120.71|109,650,449|56.82+437+441';c1[443]='10/16/2020;121.28 121.55 118.81 119.02|115,393,808|54.01+441+445';c1[448]='10/19/2020;119.96 120.42 115.66 115.98|119,366,165|49.30+446+450';c1[453]='10/20/2020;116.20 118.98 115.63 117.51|124,423,728|51.59+451+455';c1[457]='10/21/2020;116.67 118.71 116.45 116.87|89,945,980|50.56+455+459';c1[462]='10/22/2020;117.45 118.04 114.59 115.75|101,354,341|48.73+460+464';c1[467]='10/23/2020;116.39 116.55 114.28 115.04|82,572,645|47.55+465+469';c1[471]='10/26/2020;114.01 116.55 112.88 115.05|109,974,392|47.57+469+473';c1[476]='10/27/2020;115.49 117.28 114.54 116.60|90,967,291|50.59+474+478';c1[481]='10/28/2020;115.05 115.43 111.10 111.20|142,355,296|41.60+479+483';c1[485]='10/29/2020;112.37 116.93 112.20 115.32|146,129,173|49.04+483+487';c1[490]='10/30/2020;111.06 111.99 107.72 108.86|190,573,476|40.36+488+492';c1[495]='11/2/2020;109.11 110.68 107.32 108.77|122,866,899|40.26+493+497';c1[499]='11/3/2020;109.66 111.49 108.73 110.44|107,624,448|43.26+497+501';c1[504]='11/4/2020;114.14 115.59 112.35 114.95|138,235,482|50.49+502+506';c1[509]='11/5/2020;117.95 119.62 116.87 119.03|125,201,932|55.96+507+511';c1[513]='11/6/2020;118.32 119.20 116.13 118.69|114,457,922|55.42+511+515';c1[518]='11/9/2020;120.50 121.99 116.05 116.32|154,515,315|51.61+516+520';c1[523]='11/10/2020;115.55 117.59 114.13 115.97|133,345,001|51.05+521+525';c1[527]='11/11/2020;117.19 119.63 116.44 119.49|112,294,954|56.18+525+529';c1[532]='11/12/2020;119.62 120.53 118.57 119.21|101,899,286|55.68+530+534';c1[537]='11/13/2020;119.44 119.67 117.87 119.26|81,688,586|55.76+535+539';c1[541]='11/16/2020;118.92 120.99 118.15 120.30|91,183,018|57.39+539+543';c1[546]='11/17/2020;119.55 120.67 118.96 119.39|74,270,973|55.46+544+548';c1[551]='11/18/2020;118.61 119.82 118.00 118.03|74,550,131|52.62+549+553';c1[555]='11/19/2020;117.59 119.06 116.81 118.64|74,112,972|53.76+553+557';c1[560]='11/20/2020;118.64 118.77 117.29 117.34|71,568,288|50.94+558+562';c1[564]='11/23/2020;117.18 117.62 113.75 113.85|125,934,410|44.23+562+566';c1[569]='11/24/2020;113.91 115.85 112.59 115.17|113,226,248|47.07+567+571';c1[574]='11/25/2020;115.55 116.75 115.17 116.03|76,226,384|48.90+572+576';c1[578]='11/27/2020;116.57 117.49 116.22 116.59|46,691,331|50.10+576+580';c1[583]='11/30/2020;116.97 120.97 116.81 119.05|169,410,176|55.12+581+585';c1[588]='12/1/2020;121.01 123.47 120.01 122.72|125,920,963|61.36+586+590';c1[592]='12/2/2020;122.02 123.37 120.89 123.08|89,004,195|61.92+590+594';c1[597]='12/3/2020;123.52 123.78 122.21 122.94|78,967,630|61.55+595+599';c1[602]='12/4/2020;122.60 122.86 121.52 122.25|78,260,421|59.64+600+604';
    def foo(x):
        (date,rest) = x.split(";")
        (prices,volume,rsi) = rest.split("|")
        close = prices.split(" ")[3]
        volume = int(volume.replace(",", ""))
        return (date, close, volume)

    parsed = [ foo(x) for x in c1.values()]
    def insert(row):
        return f"insert into intraday_prices values (DEFAULT,1,{row[1]},{row[2]}, TIMESTAMP '{row[0]}')"
    inserts = [ insert(x) for x in parsed]
    print('\n'.join([ x[1] for x in parsed]))
    */


    let intraday_inserts = vec!["insert into intraday_prices values (DEFAULT,1,83.37,95407900, TIMESTAMP '6/8/2020')",
                                "insert into intraday_prices values (DEFAULT,1,86.00,147712364, TIMESTAMP '6/9/2020')",
                                "insert into intraday_prices values (DEFAULT,1,88.21,165423144, TIMESTAMP '6/10/2020')",
                                "insert into intraday_prices values (DEFAULT,1,83.98,198270700, TIMESTAMP '6/11/2020')",
                                "insert into intraday_prices values (DEFAULT,1,84.70,200146052, TIMESTAMP '6/12/2020')",
                                "insert into intraday_prices values (DEFAULT,1,85.75,138808920, TIMESTAMP '6/15/2020')",
                                "insert into intraday_prices values (DEFAULT,1,88.02,164436052, TIMESTAMP '6/16/2020')",
                                "insert into intraday_prices values (DEFAULT,1,87.90,114406504, TIMESTAMP '6/17/2020')",
                                "insert into intraday_prices values (DEFAULT,1,87.93,94421232, TIMESTAMP '6/18/2020')",
                                "insert into intraday_prices values (DEFAULT,1,87.43,264475808, TIMESTAMP '6/19/2020')",
                                "insert into intraday_prices values (DEFAULT,1,89.72,134038636, TIMESTAMP '6/22/2020')",
                                "insert into intraday_prices values (DEFAULT,1,91.63,211257012, TIMESTAMP '6/23/2020')",
                                "insert into intraday_prices values (DEFAULT,1,90.02,190982340, TIMESTAMP '6/24/2020')",
                                "insert into intraday_prices values (DEFAULT,1,91.21,136222032, TIMESTAMP '6/25/2020')",
                                "insert into intraday_prices values (DEFAULT,1,88.41,205256844, TIMESTAMP '6/26/2020')",
                                "insert into intraday_prices values (DEFAULT,1,90.45,129839052, TIMESTAMP '6/29/2020')",
                                "insert into intraday_prices values (DEFAULT,1,91.20,139190968, TIMESTAMP '6/30/2020')",
                                "insert into intraday_prices values (DEFAULT,1,91.03,109052836, TIMESTAMP '7/1/2020')",
                                "insert into intraday_prices values (DEFAULT,1,91.03,111057880, TIMESTAMP '7/2/2020')",
                                "insert into intraday_prices values (DEFAULT,1,93.46,115944308, TIMESTAMP '7/6/2020')",
                                "insert into intraday_prices values (DEFAULT,1,93.17,112827996, TIMESTAMP '7/7/2020')",
                                "insert into intraday_prices values (DEFAULT,1,95.34,115523504, TIMESTAMP '7/8/2020')",
                                "insert into intraday_prices values (DEFAULT,1,95.68,123026536, TIMESTAMP '7/9/2020')",
                                "insert into intraday_prices values (DEFAULT,1,95.92,90257320, TIMESTAMP '7/10/2020')",
                                "insert into intraday_prices values (DEFAULT,1,95.48,188132988, TIMESTAMP '7/13/2020')",
                                "insert into intraday_prices values (DEFAULT,1,97.06,166204064, TIMESTAMP '7/14/2020')",
                                "insert into intraday_prices values (DEFAULT,1,97.73,150660352, TIMESTAMP '7/15/2020')",
                                "insert into intraday_prices values (DEFAULT,1,96.52,107259048, TIMESTAMP '7/16/2020')",
                                "insert into intraday_prices values (DEFAULT,1,96.33,92187020, TIMESTAMP '7/17/2020')",
                                "insert into intraday_prices values (DEFAULT,1,98.36,89588832, TIMESTAMP '7/20/2020')",
                                "insert into intraday_prices values (DEFAULT,1,97.00,102509388, TIMESTAMP '7/21/2020')",
                                "insert into intraday_prices values (DEFAULT,1,97.27,82360080, TIMESTAMP '7/22/2020')",
                                "insert into intraday_prices values (DEFAULT,1,92.85,197004432, TIMESTAMP '7/23/2020')",
                                "insert into intraday_prices values (DEFAULT,1,92.62,185438864, TIMESTAMP '7/24/2020')",
                                "insert into intraday_prices values (DEFAULT,1,94.81,117879876, TIMESTAMP '7/27/2020')",
                                "insert into intraday_prices values (DEFAULT,1,93.25,101245172, TIMESTAMP '7/28/2020')",
                                "insert into intraday_prices values (DEFAULT,1,95.04,87272372, TIMESTAMP '7/29/2020')",
                                "insert into intraday_prices values (DEFAULT,1,96.19,126407408, TIMESTAMP '7/30/2020')",
                                "insert into intraday_prices values (DEFAULT,1,106.26,374295468, TIMESTAMP '7/31/2020')",
                                "insert into intraday_prices values (DEFAULT,1,108.94,307269208, TIMESTAMP '8/3/2020')",
                                "insert into intraday_prices values (DEFAULT,1,109.67,172792368, TIMESTAMP '8/4/2020')",
                                "insert into intraday_prices values (DEFAULT,1,110.06,120102788, TIMESTAMP '8/5/2020')",
                                "insert into intraday_prices values (DEFAULT,1,113.90,202428900, TIMESTAMP '8/6/2020')",
                                "insert into intraday_prices values (DEFAULT,1,111.11,198045612, TIMESTAMP '8/7/2020')",
                                "insert into intraday_prices values (DEFAULT,1,112.73,212403424, TIMESTAMP '8/10/2020')",
                                "insert into intraday_prices values (DEFAULT,1,109.38,185915844, TIMESTAMP '8/11/2020')",
                                "insert into intraday_prices values (DEFAULT,1,113.01,164514552, TIMESTAMP '8/12/2020')",
                                "insert into intraday_prices values (DEFAULT,1,115.01,201950304, TIMESTAMP '8/13/2020')",
                                "insert into intraday_prices values (DEFAULT,1,114.91,165565208, TIMESTAMP '8/14/2020')",
                                "insert into intraday_prices values (DEFAULT,1,114.61,117725656, TIMESTAMP '8/17/2020')",
                                "insert into intraday_prices values (DEFAULT,1,115.56,104095732, TIMESTAMP '8/18/2020')",
                                "insert into intraday_prices values (DEFAULT,1,115.71,144547048, TIMESTAMP '8/19/2020')",
                                "insert into intraday_prices values (DEFAULT,1,118.28,125313344, TIMESTAMP '8/20/2020')",
                                "insert into intraday_prices values (DEFAULT,1,124.37,338054640, TIMESTAMP '8/21/2020')",
                                "insert into intraday_prices values (DEFAULT,1,125.86,343339388, TIMESTAMP '8/24/2020')",
                                "insert into intraday_prices values (DEFAULT,1,124.83,208941716, TIMESTAMP '8/25/2020')",
                                "insert into intraday_prices values (DEFAULT,1,126.52,161004836, TIMESTAMP '8/26/2020')",
                                "insert into intraday_prices values (DEFAULT,1,125.01,154146696, TIMESTAMP '8/27/2020')",
                                "insert into intraday_prices values (DEFAULT,1,124.81,187629916, TIMESTAMP '8/28/2020')",
                                "insert into intraday_prices values (DEFAULT,1,129.04,223505733, TIMESTAMP '8/31/2020')",
                                "insert into intraday_prices values (DEFAULT,1,134.18,150698946, TIMESTAMP '9/1/2020')",
                                "insert into intraday_prices values (DEFAULT,1,131.40,198233370, TIMESTAMP '9/2/2020')",
                                "insert into intraday_prices values (DEFAULT,1,120.88,250623616, TIMESTAMP '9/3/2020')",
                                "insert into intraday_prices values (DEFAULT,1,120.96,332607163, TIMESTAMP '9/4/2020')",
                                "insert into intraday_prices values (DEFAULT,1,112.82,226149287, TIMESTAMP '9/8/2020')",
                                "insert into intraday_prices values (DEFAULT,1,117.32,172144189, TIMESTAMP '9/9/2020')",
                                "insert into intraday_prices values (DEFAULT,1,113.49,182274391, TIMESTAMP '9/10/2020')",
                                "insert into intraday_prices values (DEFAULT,1,112.00,180860325, TIMESTAMP '9/11/2020')",
                                "insert into intraday_prices values (DEFAULT,1,115.36,138248838, TIMESTAMP '9/14/2020')",
                                "insert into intraday_prices values (DEFAULT,1,115.54,182985051, TIMESTAMP '9/15/2020')",
                                "insert into intraday_prices values (DEFAULT,1,112.13,153804096, TIMESTAMP '9/16/2020')",
                                "insert into intraday_prices values (DEFAULT,1,110.34,176909480, TIMESTAMP '9/17/2020')",
                                "insert into intraday_prices values (DEFAULT,1,106.84,284477372, TIMESTAMP '9/18/2020')",
                                "insert into intraday_prices values (DEFAULT,1,110.08,193321297, TIMESTAMP '9/21/2020')",
                                "insert into intraday_prices values (DEFAULT,1,111.81,181592150, TIMESTAMP '9/22/2020')",
                                "insert into intraday_prices values (DEFAULT,1,107.12,148510882, TIMESTAMP '9/23/2020')",
                                "insert into intraday_prices values (DEFAULT,1,108.22,167743349, TIMESTAMP '9/24/2020')",
                                "insert into intraday_prices values (DEFAULT,1,112.28,149981441, TIMESTAMP '9/25/2020')",
                                "insert into intraday_prices values (DEFAULT,1,114.96,136441425, TIMESTAMP '9/28/2020')",
                                "insert into intraday_prices values (DEFAULT,1,114.09,98636389, TIMESTAMP '9/29/2020')",
                                "insert into intraday_prices values (DEFAULT,1,115.81,141667323, TIMESTAMP '9/30/2020')",
                                "insert into intraday_prices values (DEFAULT,1,116.79,114958231, TIMESTAMP '10/1/2020')",
                                "insert into intraday_prices values (DEFAULT,1,113.02,144711986, TIMESTAMP '10/2/2020')",
                                "insert into intraday_prices values (DEFAULT,1,116.50,105392594, TIMESTAMP '10/5/2020')",
                                "insert into intraday_prices values (DEFAULT,1,113.16,159519440, TIMESTAMP '10/6/2020')",
                                "insert into intraday_prices values (DEFAULT,1,115.08,96135446, TIMESTAMP '10/7/2020')",
                                "insert into intraday_prices values (DEFAULT,1,114.97,82322071, TIMESTAMP '10/8/2020')",
                                "insert into intraday_prices values (DEFAULT,1,116.97,100506865, TIMESTAMP '10/9/2020')",
                                "insert into intraday_prices values (DEFAULT,1,124.40,236262242, TIMESTAMP '10/12/2020')",
                                "insert into intraday_prices values (DEFAULT,1,121.10,260497701, TIMESTAMP '10/13/2020')",
                                "insert into intraday_prices values (DEFAULT,1,121.19,150268765, TIMESTAMP '10/14/2020')",
                                "insert into intraday_prices values (DEFAULT,1,120.71,109650449, TIMESTAMP '10/15/2020')",
                                "insert into intraday_prices values (DEFAULT,1,119.02,115393808, TIMESTAMP '10/16/2020')",
                                "insert into intraday_prices values (DEFAULT,1,115.98,119366165, TIMESTAMP '10/19/2020')",
                                "insert into intraday_prices values (DEFAULT,1,117.51,124423728, TIMESTAMP '10/20/2020')",
                                "insert into intraday_prices values (DEFAULT,1,116.87,89945980, TIMESTAMP '10/21/2020')",
                                "insert into intraday_prices values (DEFAULT,1,115.75,101354341, TIMESTAMP '10/22/2020')",
                                "insert into intraday_prices values (DEFAULT,1,115.04,82572645, TIMESTAMP '10/23/2020')",
                                "insert into intraday_prices values (DEFAULT,1,115.05,109974392, TIMESTAMP '10/26/2020')",
                                "insert into intraday_prices values (DEFAULT,1,116.60,90967291, TIMESTAMP '10/27/2020')",
                                "insert into intraday_prices values (DEFAULT,1,111.20,142355296, TIMESTAMP '10/28/2020')",
                                "insert into intraday_prices values (DEFAULT,1,115.32,146129173, TIMESTAMP '10/29/2020')",
                                "insert into intraday_prices values (DEFAULT,1,108.86,190573476, TIMESTAMP '10/30/2020')",
                                "insert into intraday_prices values (DEFAULT,1,108.77,122866899, TIMESTAMP '11/2/2020')",
                                "insert into intraday_prices values (DEFAULT,1,110.44,107624448, TIMESTAMP '11/3/2020')",
                                "insert into intraday_prices values (DEFAULT,1,114.95,138235482, TIMESTAMP '11/4/2020')",
                                "insert into intraday_prices values (DEFAULT,1,119.03,125201932, TIMESTAMP '11/5/2020')",
                                "insert into intraday_prices values (DEFAULT,1,118.69,114457922, TIMESTAMP '11/6/2020')",
                                "insert into intraday_prices values (DEFAULT,1,116.32,154515315, TIMESTAMP '11/9/2020')",
                                "insert into intraday_prices values (DEFAULT,1,115.97,133345001, TIMESTAMP '11/10/2020')",
                                "insert into intraday_prices values (DEFAULT,1,119.49,112294954, TIMESTAMP '11/11/2020')",
                                "insert into intraday_prices values (DEFAULT,1,119.21,101899286, TIMESTAMP '11/12/2020')",
                                "insert into intraday_prices values (DEFAULT,1,119.26,81688586, TIMESTAMP '11/13/2020')",
                                "insert into intraday_prices values (DEFAULT,1,120.30,91183018, TIMESTAMP '11/16/2020')",
                                "insert into intraday_prices values (DEFAULT,1,119.39,74270973, TIMESTAMP '11/17/2020')",
                                "insert into intraday_prices values (DEFAULT,1,118.03,74550131, TIMESTAMP '11/18/2020')",
                                "insert into intraday_prices values (DEFAULT,1,118.64,74112972, TIMESTAMP '11/19/2020')",
                                "insert into intraday_prices values (DEFAULT,1,117.34,71568288, TIMESTAMP '11/20/2020')",
                                "insert into intraday_prices values (DEFAULT,1,113.85,125934410, TIMESTAMP '11/23/2020')",
                                "insert into intraday_prices values (DEFAULT,1,115.17,113226248, TIMESTAMP '11/24/2020')",
                                "insert into intraday_prices values (DEFAULT,1,116.03,76226384, TIMESTAMP '11/25/2020')",
                                "insert into intraday_prices values (DEFAULT,1,116.59,46691331, TIMESTAMP '11/27/2020')",
                                "insert into intraday_prices values (DEFAULT,1,119.05,169410176, TIMESTAMP '11/30/2020')",
                                "insert into intraday_prices values (DEFAULT,1,122.72,125920963, TIMESTAMP '12/1/2020')",
                                "insert into intraday_prices values (DEFAULT,1,123.08,89004195, TIMESTAMP '12/2/2020')",
                                "insert into intraday_prices values (DEFAULT,1,122.94,78967630, TIMESTAMP '12/3/2020')",
                                "insert into intraday_prices values (DEFAULT,1,122.25,78260421, TIMESTAMP '12/4/2020')"];

    let inserts = intraday_inserts.into_iter().map(|stock| {
        sqlx::query(stock)
            .execute(conn)
    });

    futures::future::join_all(inserts).await.into_iter().fold(
        sqlx::Result::Ok(sqlx::postgres::PgDone::default()),
        |acc, curr| {
            if Result::is_ok(&curr) {
                acc
            } else {
                curr
            }
        },
    )
}
