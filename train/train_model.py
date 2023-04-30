from sklearn import datasets
from catboost import CatBoostClassifier, metrics

iris = datasets.load_iris()

X = iris.data
y = iris.target

model = CatBoostClassifier(
    custom_loss=[metrics.Accuracy()],
    random_seed=42,
)

model.fit(X, y)
model.save_model("model.cbm")
