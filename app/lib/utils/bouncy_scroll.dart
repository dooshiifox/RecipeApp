import 'package:flutter/material.dart';
import 'dart:math' as math;

// https://techsolutionshere.com/how-to-use-clampingscrollphysics-in-top-and-bouncingscrollphysics-in-bottom-on-a-scroll-list/

class CustomBouncingScrollPhysics extends BouncingScrollPhysics {
  /// The spring force after reaching the top of the container.
  /// Higher values will result in a weaker resistance force.
  final double topFactor;

  /// The spring force after reaching the bottom of the container.
  /// Higher values will result in a weaker resistance force.
  final double bottomFactor;

  const CustomBouncingScrollPhysics(
      {this.topFactor = 1, this.bottomFactor = 1, ScrollPhysics? parent})
      : super(parent: parent);

  @override
  CustomBouncingScrollPhysics applyTo(ScrollPhysics? ancestor) {
    return CustomBouncingScrollPhysics(
      topFactor: topFactor,
      bottomFactor: bottomFactor,
      parent: buildParent(ancestor),
    );
  }

  @override
  double frictionFactor(double overscrollFraction) =>
      0.52 * math.pow(1 - overscrollFraction, 2);

  @override
  double applyPhysicsToUserOffset(ScrollMetrics position, double offset) {
    assert(offset != 0.0);
    assert(position.minScrollExtent <= position.maxScrollExtent);

    if (!position.outOfRange) return offset;

    final double overscrollPastStart =
        math.max(position.minScrollExtent - position.pixels, 0.0);
    final double overscrollPastEnd =
        math.max(position.pixels - position.maxScrollExtent, 0.0);
    final double overscrollPast =
        math.max(overscrollPastStart, overscrollPastEnd);
    final bool easing = (overscrollPastStart > 0.0 && offset < 0.0) ||
        (overscrollPastEnd > 0.0 && offset > 0.0);

    final double friction = easing
        // Apply less resistance when easing the overscroll vs tensioning.
        ? frictionFactor(
            (overscrollPast - offset.abs()) / position.viewportDimension)
        : frictionFactor(overscrollPast / position.viewportDimension);
    final double direction = offset.sign;

    double r =
        direction * _applyFriction(overscrollPast, offset.abs(), friction);
    if (overscrollPastStart > 0) {
      r *= topFactor;
    } else if (overscrollPastEnd > 0) {
      r *= bottomFactor;
    }
    return r;
  }

  static double _applyFriction(
      double extentOutside, double absDelta, double gamma) {
    assert(absDelta > 0);
    double total = 0.0;
    if (extentOutside > 0) {
      final double deltaToLimit = extentOutside / gamma;
      if (absDelta < deltaToLimit) return absDelta * gamma;
      total += extentOutside;
      absDelta -= deltaToLimit;
    }
    return total + absDelta;
  }

  @override
  Simulation? createBallisticSimulation(
      ScrollMetrics position, double velocity) {
    final Tolerance tolerance = this.tolerance;
    if (velocity.abs() >= tolerance.velocity || position.outOfRange) {
      var v = velocity;
      if (position.minScrollExtent - position.pixels > 0) {
        v *= topFactor;
      } else if (position.pixels - position.maxScrollExtent > 0) {
        v *= bottomFactor;
      }
      return BouncingScrollSimulation(
        spring: spring,
        position: position.pixels,
        velocity: v,
        leadingExtent: position.minScrollExtent,
        trailingExtent: position.maxScrollExtent,
        tolerance: tolerance,
      );
    }
    return null;
  }
}

/// A scroll that has a custom bounce effect.
class BouncyScroll extends StatefulWidget {
  /// The child of the scrollable.
  final Widget child;

  /// The spring force after reaching the top of the container.
  /// Higher values will result in a weaker resistance force.
  final double? topFactor;

  /// The spring force after reaching the bottom of the container.
  /// Higher values will result in a weaker resistance force.
  final double? bottomFactor;

  const BouncyScroll(
      {required this.child, this.topFactor, this.bottomFactor, Key? key})
      : super(key: key);

  @override
  State<BouncyScroll> createState() => _BouncyScrollState();
}

class _BouncyScrollState extends State<BouncyScroll> {
  ScrollController scrollController = ScrollController();

  @override
  void dispose() {
    super.dispose();
    scrollController.dispose(); //always dispose the controller
  }

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      controller: scrollController,
      physics: CustomBouncingScrollPhysics(
        topFactor: widget.topFactor ?? 1.0,
        bottomFactor: widget.bottomFactor ?? 1.0,
      ),
      child: widget.child,
    );
  }
}
