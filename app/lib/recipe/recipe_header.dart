import 'package:flutter/material.dart';
import 'package:recipeapp/recipe/recipe.dart';

class RecipeHeader extends StatelessWidget {
  final double topPadding;

  final Recipe recipe;

  const RecipeHeader(this.recipe, {this.topPadding = 100, Key? key})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        gradient: LinearGradient(
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
          colors: [recipe.tlColor, recipe.brColor],
        ),
        borderRadius: const BorderRadius.all(Radius.circular(30)),
      ),
      padding: EdgeInsets.fromLTRB(20, topPadding, 20, 30),
      width: double.infinity,
      child: Column(children: [
        Container(
          width: double.infinity,
          height: 180,
          decoration: BoxDecoration(
            borderRadius: const BorderRadius.all(Radius.circular(30)),
            image: DecorationImage(
              image: AssetImage(recipe.imageLink),
              fit: BoxFit.cover,
            ),
          ),
        ),
        Padding(
          padding: const EdgeInsets.fromLTRB(20, 10, 20, 0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              recipe.isWeekly
                  ? const Text(
                      'Weekly Recipe',
                      style: TextStyle(
                        fontSize: 18,
                        fontWeight: FontWeight.bold,
                        color: Colors.black45,
                        letterSpacing: -0.2,
                      ),
                    )
                  : const SizedBox(),
              Text(
                recipe.title,
                style: const TextStyle(
                  fontSize: 28,
                  fontWeight: FontWeight.bold,
                  color: Colors.black87,
                  letterSpacing: -0.2,
                ),
              ),
              Padding(
                padding: const EdgeInsets.fromLTRB(16, 4, 16, 8),
                child: Text(
                  recipe.nutrientsToString(),
                  style: const TextStyle(
                    fontSize: 18,
                    fontWeight: FontWeight.bold,
                    color: Colors.black38,
                    letterSpacing: -0.2,
                  ),
                ),
              ),
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Flexible(
                    child: Row(
                      children: [
                        const Padding(
                          padding: EdgeInsets.only(left: 8.0, right: 8.0),
                          child: Icon(
                            Icons.timer_rounded,
                            size: 18,
                            color: Colors.black54,
                            semanticLabel: 'Time to Cook',
                          ),
                        ),
                        Flexible(
                          child: Text(
                            recipe.timeToCookString(),
                            style: const TextStyle(
                              fontSize: 16,
                              color: Colors.black54,
                              letterSpacing: -0.2,
                            ),
                          ),
                        ),
                      ],
                    ),
                  ),
                  Flexible(
                    child: Row(
                      children: [
                        const Padding(
                          padding: EdgeInsets.only(left: 8.0, right: 8.0),
                          child: Icon(
                            Icons.restaurant_rounded,
                            size: 18,
                            color: Colors.black54,
                          ),
                        ),
                        Flexible(
                          child: Text(
                            'Serves ${recipe.servings}',
                            style: const TextStyle(
                              fontSize: 16,
                              color: Colors.black54,
                              letterSpacing: -0.2,
                            ),
                          ),
                        ),
                      ],
                    ),
                  ),
                  recipe.ratingStars()
                ],
              )
            ],
          ),
        )
      ]),
    );
  }
}
