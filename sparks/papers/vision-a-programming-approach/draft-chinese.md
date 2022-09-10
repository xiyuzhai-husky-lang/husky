# 计算机视觉：编程方法

## 简介

### 传统计算机视觉

传统计算机视觉大概分为两步，
- 第一步是特征提取，需要针对具体的应用，利用专业知识和复杂的调参过程。
- 第二步是将提取的特征喂给经典的机器学习算法

传统的计算机视觉主要的缺点有:

- 人工成本大
- 特征的计算不够高效
- 精确度有限
- 特征表达性能有限
- 自动化程度低

### 深度学习

深度学习主要是利用数据进行自动的特征提取，能够得到深层高效的特征表示，最近在大部分的任务上取得了远超传统计算机视觉方法的准确度。

深度学习的缺点：

- 需要大量的标注数据
- 不确定性大，很难提炼出科学的规律，需要仰仗大量经验来调参
- 训练和推理需要大量的算力
- 模型比传统计算机视觉更加缺乏可解释性

### 以编程为主的新方法

本文提出以编程为主的新方法，主要的优势为：

- 所需算力大大降低。训练成本几乎等同于推理成本，而推理成本比传统计算机视觉和深度学习大大降低。在mnist上可以比Lenet快20倍。
- 具有像软件一样的强可解释性，所生成的推理程序可以看成基于规则进行推理
- 绝大部分是基于合理的数学模型，很少存在需要调参的部分
- 准确率没有上限

## 图像分类的编程方法

### 困难的降解

一个图像分类问题的困难大致有

- 类别太多
- 不具备明显的解析解
- 计算效率

下面我们来看如何降解这些困难

#### 简化为二分类问题

假设C是一个类别的集合，我们可以把判断一个输入属于C里边哪一个类的问题拆分为对C里边的每一个类判断该输入是否属于这一个类。

以mnist为例，如果我们有能判断一张图是不是0，是不是1，是不是2...的一组二分类器，他们组合在一起就可以告诉我们一张图是0到9的哪个字母。

#### 降低数据集大小

固定一个类，我们来考虑如何得到这个类对应的二分类器。很难直接得到强可解释而且高效的程序，所以我们考虑进一步将问题分解。

为此我们需要进行一个假设：将输入空间想象成连续的，数据的分布大概像是若干个聚集地的组合，每个聚集地都属于一个类，在聚集地的交界处附近的输入比较难以被分类，而远离交界处的输入容易被分类，容易到可以被一个非常直接的强解释而且高效的程序直接分离出来。

所以大致的流程是这样：先用一个非常直接而且高效的程序分离出来那些远离交界处的样本，包括正样本和负样本，然后我们只需要考虑剩下的样本。

对于剩下的样本，它们占据了输入空间更小的部分，所以对于一个固定的函数类，它在缩小了的输入空间上表达性更强。能更准确地近似基准真相。

（数学上，blabla）

所以我们可以继续找到一个直接的高效的强可解释的程序进一步分离出相对简单的样本

重复这个过程直到我们得到我们想要的准确率。

值得注意的是，这个过程与传统计算机视觉的区别：虽然在我们的方法中也需要人为特征提取，但人和机器是交互的，人提取特征机器得到结果，并以恰当的方式显示出来，人根据这个结果进一步提取更有用的特征，形成良性循环。

#### 分离统计上和计算上的困难

对于深度学习，往往存在准确率和计算效率上的权衡，算力对准确率的曲线往往非常陡峭，但我们将说明在合适的框架下并不存在准确率和计算效率的权衡。

经验上讲准确率主要取决于少数的模棱两可的样本，即那些落在聚集地交界处的样本，而平均计算效率主要取决于在常见的样本上的计算效率。如果我们有这样一个计算效率极高但局限的分类器，它可以输出一个类别或者一个不知道信号，当它输入类别的时候，准确率高达99%，而且它输出不知道信号的概率足够小，比如10%，那么这是将它和一个计算低效但准确率达到高达99%的分类器结合，先使用局限的分类器，当局限的分类器不知道的时候再用低效的分类器这样可以得到一个比低效分类器效率高10倍，但准确率不变的分类器。

### 合理的数学建模


什么是对形状的合理数学刻画，它应该满足以下所有条件，

- 首先必须有足够的表达性
- 其次计算上足够高效

很不幸的是，数学家们研究的结构并没有足够的表达性，黎曼结构过于精细，而拓扑结构则过于粗糙。

而计算机科学家发明的有足够表达性的方法则在效率上有缺陷，比如shape context 或者不具有可解释性.

我们所使用的方法实际上是受画画的素描所启发的。这一点也不奇怪，因为艺术家才是对形状或者说人对形状的感知理解最深的一群人。


what is the correct mathematical characterization of shape?it must satisfy all the following conditions

- must be sufficiently expressive 
- must be possible to be computed efficiently

unfortunately existing structures (Riemannian and Topological) studied by mathematicians are not sufficiently expressive, Riemannian structures are too refined, Topological structures are too coarse.

expressive models invented  by computer scientists such as shape-context are computationally costly, despite attempts of optimization.

so the desire model exist must significantly differ from all these characterizations.

it turns out the characterizations we use here is adapted from the technique of sketching in drawinng. In hindersight, this is not surprising because artists have been studying  shapes in a pragmatic manner for thousands of years.
