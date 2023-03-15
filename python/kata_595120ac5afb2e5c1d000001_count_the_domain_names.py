import re
import pprint

domains_list = """\
*.amazon.co.uk    89
*.doubleclick.net    139
*.fbcdn.net    212
*.in-addr.arpa    384
*.l.google.com    317
1.client-channel.google.com    110
6.client-channel.google.com    45
a.root-servers.net    1059
apis.google.com    43
clients4.google.com    71
clients6.google.com    81
connect.facebook.net    68
edge-mqtt.facebook.com    56
graph.facebook.com    150
mail.google.com    128
mqtt-mini.facebook.com    47
ssl.google-analytics.com    398
star-mini.c10r.facebook.com    46
staticxx.facebook.com    48
www.facebook.com    178
www.google.com    162
www.google-analytics.com    127
www.googleapis.com    87"""

domain2 = """\
globo.com	299
tzqwc.p03.quora.com	431
i9ip.google.co.in	32
l.globo.com	380
s9emv.linkedin.com	69
y0pks.vpc.popads.net	360
28f.google.com.co	199
qq.com	151
f4.mtp2e.diply.com	89
ej.googleusercontent.com	488
*.tumblr.com	135
x.googleusercontent.com	226
*.twitter.com	323
zlhf9j.uol.com.br	97
*.afpt8t.jd.com	476
*.3ld.taobao.com	339
*.7k1hiz.redtube.com	70
0tlq.qm.linkedin.com	337
*.a8fqk.32nsa.pornhub.com	17
*.n9.vmj.instagram.com	110
*.ga.le.cnzz.com	299
*.q63vry.twitch.tv	391
ok.ru	289
0.c66ubt.baidu.com	215
msxj.yahoo.com	76
ahg49.u3sk.coccoc.com	283
bvk.gk.google.com.br	229
walmart.com	22
xhamster.com	386
*.a1q.wzrx.imgur.com	3
z.29.hao123.com	125
*.bohsw.bing.com	116
caj.netflix.com	262
gg3ii.q.stackoverflow.com	296
*.0j8tf.naver.com	327
github.com	394
yhjksf.google.co.uk	41
o6e6k.live.com	77
blogspot.com	341
m171.microsoftonline.com	53
c9x8jg.nkuo.rakuten.co.jp	106
*.wpta.i.bet365.com	456
hnkco.0.cnzz.com	387
google.co.in	349
a6tmr6.amazon.co.jp	72
i9j.blogspot.com	467
google.de	359
4.dczzsd.google.co.in	412
0lv23.ebay.co.uk	54
*.weibo.com	313
imgur.com	478
*.o9c.bs53t.stackoverflow.com	189
jq3.diply.com	174
*.wqq2h.github.com	221
*.c.bing.com	213
*.amazon.co.jp	368
pu.z4qg.google.de	199
lw5kv.tp4.gmw.cn	219
*.z.google.co.uk	436
o.naver.com	484
r3.amazon.co.jp	230
*.5w.lfud.amazon.com	409
a.pixnet.net	318
xg.y.booking.com	303
google.co.id	185
epg.qq.com	426
mail.ru	169
*.bing.com	469
bbc.co.uk	97
v480.gmw.cn	185
6b2j8y.bt.cnzz.com	58
ft.xnxx.com	353
0mwhim.samsung.com	430
*.yahoo.co.jp	358
uso31.soso.com	272
6mbki1.nhfv.imgur.com	237
tumblr.com	19
*.lqb7nh.bd408.ebay.co.uk	223
netflix.com	363
0.github.com	311"""


def count_domains(domains, min_hits=0):
    print(domains.split("\n"))
    # regex = r"(?:[^./\s/]*)(?:\.[^\s./]{2,4}){1,2}(?=(?:\/\S*)?$)"
    # regex2 = r"([^./\s/]*)(\.[^\s./]{2,4}){1,2}(?=(?:\/\S*)?$)"
    # regex3 = r"(?:[^./\s/]*)(?:[\w-]{3,})(?:\.\w\w)?\.\w+\.?$"
    regex4 = r"(?:[^./\s/]*){0,2}(?:[\w-]{1,})(?:\.(?:com|co))?\.\w+\.?(?:/|:|$)"
    split_arr = domains.split("\n")
    # d = dict(x.split() for x in split_arr)
    d = {}
    final = {}
    filter = {}
    pp = pprint.PrettyPrinter()
    for x in split_arr:
        k, v = x.split()
        d[k] = int(v)
    # for x in split_arr:
    #     k, v = x.split()
    #     # res = re.findall(regex, k)[0]
    #     # print(res)
    #     if res in d:
    #         final[res] += int(v)
    #     else:
    #         final[res] = int(v)
    #     # if res in d:
    #     final[res] = final.get(res, int(v))

    for k, v in d.items():
        # k, v = x.split()
        # res = re.findall(regex, k)[0]
        res = "".join(re.findall(regex4, k)[0])
        print(f"res: {res} => {v}")
        # print(f"{re.findall(regex,k)}")
        # if res in d:
        #     final[res] += int(v)
        # else:
        #     final[res] = int(v)
        # if res in d:
        final[res] = final.get(res, 0) + int(v)

    pp.pprint(final)
    if min_hits == 0:
        filter = {k: v for k, v in final.items()}
    else:
        filter = {k: v for k, v in final.items() if v >= min_hits}

    # print(split_arr)
    # for x in split_arr:
    #     print(x.split())
    # for k, _ in d.items():
    #     res = re.findall(regex, k)
    #     print(res)
    # for k, v in d.items():
    #     d[k] = int(v)

    a = []
    # sort by desc order
    # sort_filter = sorted(filter, key=filter.get, reverse=True)
    # sort_filter = sorted(filter.items(), key=lambda x: (x[0], x[1]), reverse=True)

    # sort by value first and if value are the same, sorty by key (alphabetical)
    # order, the key=lamda x: (-x[1], x[0]) means to reverse the order when
    # sorting by key
    sort_filter = sorted(filter.items(), key=lambda x: (-x[1], x[0]))
    for k, v in sort_filter:
        a.append(f"{k} ({v})")
    # for x in sort_filter:
    #     a.append(f"{x} ({filter[x]})")

    # pp.pprint(sort_filter)
    pp.pprint("\n".join(a))


def main():
    # count_domains(domain2, 500)
    count_domains(domains_list, 500)


if __name__ == "__main__":
    main()


# some regex
# PATTERN = re.compile(r'([^.\n]+(\.com?)?\.\w*)\s+(\d+)')
# REG = re.compile(r'(?P<domain>[^.\s]+(?:\.co|\.com)?\.[^.\s]+)\s+(?P<hits>\d+)')
