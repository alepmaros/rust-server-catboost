from sklearn import datasets
from catboost import CatBoostClassifier, metrics

iris = datasets.load_iris()

X = iris.data
y = iris.target

print(X, y)

model = CatBoostClassifier(
    custom_loss=[metrics.Accuracy()],
    random_seed=42,
)

model.fit(X, y)

print(X[0])
print(model.pred(X[0]))

model.save_model("model.cbm")
