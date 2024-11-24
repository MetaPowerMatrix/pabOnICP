# 原力小镇
- 构建一个AI驱动的在ICP网络上运行的社交应用
- 验证ICP网络运行社交应用的可行性
- How does it look like in action? Have any screenshots, videos etc.?
-[ Web App访问地址](https://tsoel-7yaaa-aaaai-alcva-cai.icp0.io/)

## Introduction
This is a high level description of the project. Describe what the project is for, what it is doing and which problem it solves. This should not be long, usually 2-3 lines is good, keep it short and precise.

Highlight some features:
- 话题社交。
- 和不同的AI角色聊天。
- 和不同的真人角色聊天。

Adding an illustration of the architecture can quickly explain how the project is built, and how it works. 

![Illustration Example](local-workflow.png)

## Installation

### Prerequisites
需要先安装ICP的开发工具

```bash
$ sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```

### Install
下载和编译部署cansiter的步骤

```bash
$ git clone [<GitHub repo>](https://github.com/MetaPowerMatrix/pabOnICP)
$ cd pabOnIcp
$ dfx deploy --network ic
```
部署之前需要本地钱包中有足够的cycles

## Usage
部署完成之后，可以通过web界面访问canister的接口，也可以使用dfx工具反问接口

### Example 1
Usage examples can be canister calls:

```bash
$ dfx canister call matrix hi
$ dfx canister call agent request_all_patos
```

## Documentation
Further documentation can exist in the README file if the project only contains a few functions. It can also be located elsewhere, if it is extensive, if this is the case, link to it.  

## Roadmap
Describe the project roadmap, this could be the grant milestones, but it could also be the team's broader project roadmap.

- [x] 移植Matrix和Agent服务，部署两个canister
- [x] 移植Pato智能体服务，部署一个canister 
- [x] 增加向量数据库使用，部署一个canister. 
- [x] 部署前端App到canister，可以通过canister域名访问应用的功能.
- [ ] 添加更多的系统AI角色.
- [ ] 构建数据量足够大的社交应用向量数据库供检索.

## License
This project is licensed under the MIT license, see LICENSE.md for details. See CONTRIBUTE.md for details about how to contribute to this project. 


## References
- [Internet Computer](https://internetcomputer.org)




Party Board的三个服务，matrix，agent和battery在ICP网络上的实现，编译部署成三个canister
