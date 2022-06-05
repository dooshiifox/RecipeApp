import 'package:flutter/material.dart';
import 'package:recipeapp/recipe/ingredient.dart';

class Recipe {
  final String title;
  final String imageLink;
  final List<Ingredient> ingredients;
  final Color tlColor;
  final Color brColor;
  final bool isWeekly;
  final List<String> nutrients;
  final int rating;
  final int timeToCook;
  final int servings;

  const Recipe(
    this.title, {
    required this.imageLink,
    required this.ingredients,
    required this.nutrients,
    required this.rating,
    required this.timeToCook,
    required this.servings,
    this.isWeekly = false,
    this.tlColor = const Color(0xFF525A66),
    this.brColor = const Color(0xFF3a3d41),
  });

  String nutrientsToString() {
    if (nutrients.isEmpty) {
      // Contains... no nutrients?
      return 'Contains... no nutrients?';
    } else if (nutrients.length == 1) {
      // Contains A
      return 'Contains ${nutrients[0]}';
    } else if (nutrients.length == 2) {
      // Contains A & B
      return 'Contains ${nutrients[0]} & ${nutrients[1]}';
    } else {
      // Contains A, B, ..., Y, and Z
      return 'Contains ${nutrients.sublist(0, nutrients.length - 2).join(', ')}, and ${nutrients[nutrients.length - 1]}';
    }
  }

  String timeToCookString() {
    // timeToCook is in minutes. Convert to pleasant format
    if (timeToCook <= 1) {
      return 'A minute!';
    } else if (timeToCook < 60) {
      return '$timeToCook mins';
    } else if (timeToCook < 120) {
      return '1 hour';
    } else if (timeToCook < 1440) {
      return '${(timeToCook / 60).round()} hours';
    } else {
      return 'Over a day?!';
    }
  }

  Widget ratingStars() {
    // Rating is out of 5. Convert to stars
    return Row(
      children: List.generate(5, (index) {
        return Container(
          // First star is translated to the right by 16px, second by 12px, so on.
          // Needed because otherwise the stars are too far apart.
          // Does fuck with things so fix if possible!
          transform: Matrix4.translationValues((4 - index) * 4, 0, 0),
          child: Icon(
            index < rating
                ? Icons.star_rate_rounded
                : Icons.star_border_rounded,
            size: 24,
            color: Colors.black54,
          ),
        );
      }),
    );
  }
}
