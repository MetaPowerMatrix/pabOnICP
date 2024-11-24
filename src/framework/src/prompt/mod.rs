pub mod gen;

pub static PREDEFINED_TAGS: &str = r#"
    [
        "完美主义者",
        "干饭人干饭魂",
        "懒癌晚期",
        "起床困难",
        "躺平",
        "丢进人群会消失",
        "话痨",
        "社恐",
        "中二病患者",
        "共情能力强",
        "拖延症患者",
        "弱智吧吧主竞选者",
        "反射弧能绕地球一周",
        "保温杯里泡枸杞",
        "满满的正能量",
        "有点玉玉",
        "烂梗大王",
        "地狱笑话集",
        "双面不粘锅",
        "活到老学到老",
        "恋爱脑",
        "夜猫子",
        "出院！",
        "白來孰",
        "笑点低",
        "靠谱",
        "理性",
        "感性",
        "勇敢",
        "细腻",
        "我爱说实话",
        "杠精",
        "最爱独处",
        "逻辑鬼才",
        "先天搭子圣体",
        "没有耳机会死星人",
        "学渣",
        "学霸",
        "coser",
        "不找对象",
        "钢铁直男",
        "吃谷人",
        "大学生",
        "高中生",
        "初中生",
        "研究生在读",
        "打工人",
        "老二次元",
        "吃货",
        "同人圈写手",
        "互联网民工",
        "摄影师",
        "医生",
        "海投简历",
        "手工达人",
        "舞见",
        "考证",
        "纯文科生",
        "纯理科生",
        "画师",
        "铲屎官",
        "腐女",
        "留学生",
        "考公考编",
        "考研党",
        "2.5次元",
        "游戏宅",
        "资深任豚",
        "己婚",
        "（一米八）",
        "团宠",
        "多才多艺",
        "自媒体人",
        "永远在路上",
        "博主",
        "设计师",
        "尾款人",
        "上过国服",
        "写过小说",
        "拿过奖学金",
        "进过年级前50",
        "经历过裁员",
        "出过国",
        "画过漫画",
        "跑过马拉松",
        "精通多门语言",
        "做过游戏",
        "上过电视",
        "看过流星雨",
        "摆过摊",
        "发过顶刊",
        "救过人",
        "日常背单词",
        "调戏过骗子",
        "在考四六级",
        "教资备考中",
        "单抽出过金",
        "减过肥",
        "买过比特币",
        "猫狗双全",
        "看过日出",
        "日常住校",
        "当过社团干部",
        "写过歌",
        "当过up主",
        "创过业",
        "作过诗",
        "离异带俩娃",
        "飞过无人机",
        "留过学",
        "挂过科",
        "休学",
        "三坑玩家",
        "聊天",
        "随手拍",
        "嗦粉",
        "什么都懂一点",
        "树洞",
        "八卦吃瓜",
        "户外运动",
        "写小说",
        "做手工",
        "漫展",
        "绘画",
        "KTV麦霸",
        "CPDD",
        "练字书法",
        "游山玩水",
        "消息秒回",
        "减肥",
        "健身",
        "电竞赛事",
        "美食烹饪",
        "体育赛事",
        "数码产品",
        "技术宅",
        "把音乐刻进DNA",
        "通晓中华上下五千年",
        "吐槽狂魔",
        "美妆",
        "穿搭",
        "炸鸡汉堡",
        "自拍",
        "City Walk",
        "跳舞",
        "声控",
        "撸猫",
        "混饭圈",
        "爱看网文",
        "熬夜",
        "讨厌数学",
        "华流就是最diao的",
        "流行音乐",
        "周杰伦",
        "网易云",
        "QQ音乐",
        "钢琴",
        "吉他",
        "乐队",
        "livehouse",
        "KTV",
        "粵语歌",
        "J_POP",
        "K-POP",
        "唱见",
        "架子鼓",
        "古筝",
        "演唱会",
        "摇滚",
        "电音",
        "王者荣耀",
        "和平精英",
        "LOL",
        "原神",
        "蛋仔派对",
        "光•遇",
        "金铲铲之战",
        "Steam",
        "Switch",
        "音游",
        "乙女游戏",
        "宝可梦",
        "英雄联盟手游",
        "第五人格",
        "我的世界",
        "休闲游戏",
        "PS5",
        "明日方舟",
        "无期迷途",
        "重返未来：1999",
        "模拟经营游戏",
        "永劫无间",
        "无畏契约",
        "双人成行",
        "塞尔达是天",
        "P5天下第一",
        "怪物猎人",
        "奥本海默",
        "烧脑",
        "真人秀",
        "纪录片",
        "流浪地球",
        "泰剧",
        "港台剧",
        "动漫",
        "星际穿越",
        "DC",
        "漫威",
        "好莱坞电影",
        "网剧",
        "韩剧",
        "英美剧",
        "日剧",
        "综艺",
        "国产剧",
        "世界杯",
        "NBA",
        "CBA",
        "羽毛球",
        "跳绳",
        "減肥",
        "健身增肌",
        "健身房",
        "滑雪",
        "游泳",
        "爬山",
        "瑜伽",
        "骑行",
        "滑板",
        "跑步",
        "足球",
        "篮球",
        "露营",
        "棒球",
        "书单推荐",
        "种田基建",
        "校园",
        "克苏鲁",
        "无限流",
        "魔幻",
        "刘慈欣",
        "鲁迅",
        "红楼梦",
        "三国",
        "哈利波特",
        "村上春树",
        "东野圭吾",
        "科幻",
        "推理哲学",
        "历史",
        "散文",
        "名著",
        "泡图书馆",
        "古文",
        "网络小说",
        "手办尾款人",
        "蓝色监狱",
        "排球少年",
        "宫崎骏老爷子YYDS",
        "出没各地漫展",
        "梦想是成为马猴烧酒",
        "葬送的芙莉莲",
        "OC人",
        "主播女孩重度依赖",
        "全职高手",
        "支持国产动漫",
        "原著党",
        "京阿尼一生推",
        "咒术回战",
        "民工漫",
        "新海诚",
        "漫画",
        "我将以高达形态出击",
        "人活着就是为了…",
        "（已黑化）",
        "CV厨",
        "轻小说",
        "福瑞控",
        "柚子厨",
        "18禁",
        "无聊",
        "孤独寂寞冷",
        "KOL"
    ]
"#;